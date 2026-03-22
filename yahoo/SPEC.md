# Yahoo Messenger — Rust Recreation Spec

A faithful recreation of the classic Yahoo Messenger v5.x–6.x (2001–2003) desktop client and server, built entirely in Rust.

## 1. Overview

**Target era:** Yahoo Messenger v5–6, the iconic grey/blue UI with simple buddy list, separate chat windows, animated emoticons, audibles, and buzzes.

**Visual philosophy:** Faithful but modern — same color palette, layout structure, and visual language as the original, but rendered with clean modern techniques. No fake Win32 bevels or XP-era chrome; the spirit of the original without pixel-level imitation.

**Scope:** Full-stack messaging application — a custom Rust server and a native desktop client — delivering 1-on-1 real-time chat with the signature Yahoo Messenger features: buzz/nudge, custom emoticons, presence/status, avatars, system tray integration, and sound effects.

---

## 2. Architecture

### Cargo Workspace

The project is structured as a Cargo workspace with three crates:

```
yahoo/
├── Cargo.toml          # workspace root
├── client/             # desktop GUI client (iced 0.14)
│   ├── Cargo.toml
│   └── src/
├── server/             # WebSocket server (tokio + axum)
│   ├── Cargo.toml
│   └── src/
├── shared/             # message types, protocol definitions, constants
│   ├── Cargo.toml
│   └── src/
└── assets/             # emoticons, sounds, avatars (shared by client)
```

- **`shared`** — Contains all protocol message types (`serde` serializable structs/enums), status enums, error codes, and constants. Both client and server depend on this crate.
- **`server`** — Standalone binary. Runs the WebSocket server, manages SQLite database, handles auth, presence, message routing, and offline queuing.
- **`client`** — Desktop application binary. Iced 0.14 multi-window GUI with local SQLite cache, audio playback, and system tray integration.

### Key Dependencies

| Crate | Purpose |
|---|---|
| `iced` 0.14 (`canvas`, `tokio`) | GUI framework, multi-window via `iced::daemon` |
| `iced_aw` 0.13 | Extended widget library |
| `tokio` | Async runtime (both client and server) |
| `axum` | HTTP/WebSocket server framework |
| `tokio-tungstenite` | Client-side WebSocket |
| `serde` + `serde_json` | JSON serialization for protocol messages |
| `rusqlite` | SQLite for server storage and client-side cache |
| `rodio` | Audio playback (sound effects) |
| `argon2` | Password hashing |
| `tray-icon` + `winit` | System tray integration |
| `image` | Avatar/emoticon image handling |

---

## 3. Networking & Protocol

### Transport

- **WebSocket** over plain `ws://` (no TLS/encryption).
- Single persistent WebSocket connection per client session.
- JSON-encoded messages on the wire.

### Message Envelope

Every message over the WebSocket uses a common envelope:

```rust
// shared/src/protocol.rs
#[derive(Serialize, Deserialize)]
struct Envelope {
    msg_type: MessageType,
    payload: serde_json::Value,
    timestamp: u64,          // unix millis
    request_id: Option<u32>, // for request/response correlation
}
```

### Message Types

```rust
enum MessageType {
    // Auth
    LoginRequest,
    LoginResponse,
    RegisterRequest,
    RegisterResponse,
    LogoutNotify,
    KickedNotify,           // "signed in from another location"

    // Presence
    StatusChange,
    PresenceUpdate,         // server → client: a contact's status changed
    PresenceBulk,           // server → client: initial full contact status list

    // Contacts
    AddContactRequest,
    AddContactResponse,
    ContactRequestReceived, // server → client: someone wants to add you
    ContactRequestAccept,
    ContactRequestDeny,
    ContactRemove,
    ContactListSync,        // server → client: full contact list on login

    // Buddy List Organization
    GroupCreate,
    GroupRename,
    GroupDelete,
    GroupMoveContact,

    // Chat
    ChatMessage,
    ChatMessageAck,
    TypingStart,
    TypingStop,
    BuzzSend,
    BuzzReceived,

    // Offline
    OfflineMessages,        // server → client: queued messages delivered on login

    // Avatar
    AvatarChange,
    AvatarSync,

    // System
    Error,
    Pong,
    Ping,
}
```

### Session Management

- Client sends `LoginRequest` with username + password.
- Server validates credentials, creates a session, and responds with `LoginResponse` containing a session token and the user's contact list.
- If the account is already logged in elsewhere, the previous session is disconnected with a `KickedNotify` message before the new session is established.
- The session token is included in the WebSocket URL query string on reconnect (not used for auto-reconnect — see Connection Handling).

---

## 4. Server

### Overview

A standalone Rust binary using `tokio` + `axum`. Single-process, single-server architecture suitable for small-scale deployments (friend groups, LAN parties, hobby use).

### Endpoints

- `GET /ws` — WebSocket upgrade endpoint (main protocol connection)
- `GET /avatar/{username}` — Serve avatar images (HTTP, not WebSocket)

### SQLite Schema

```sql
-- User accounts
CREATE TABLE users (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    username    TEXT UNIQUE NOT NULL,
    password    TEXT NOT NULL,  -- argon2 hash
    avatar_id   TEXT DEFAULT 'default',
    status      TEXT DEFAULT 'offline',
    custom_msg  TEXT DEFAULT '',
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Contact relationships
CREATE TABLE contacts (
    user_id    INTEGER REFERENCES users(id),
    contact_id INTEGER REFERENCES users(id),
    group_name TEXT DEFAULT 'Friends',
    PRIMARY KEY (user_id, contact_id)
);

-- Pending friend requests
CREATE TABLE contact_requests (
    from_user  INTEGER REFERENCES users(id),
    to_user    INTEGER REFERENCES users(id),
    message    TEXT DEFAULT '',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (from_user, to_user)
);

-- Buddy list groups
CREATE TABLE groups (
    user_id    INTEGER REFERENCES users(id),
    group_name TEXT NOT NULL,
    sort_order INTEGER DEFAULT 0,
    PRIMARY KEY (user_id, group_name)
);

-- Message history (also serves as offline queue)
CREATE TABLE messages (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    from_user   INTEGER REFERENCES users(id),
    to_user     INTEGER REFERENCES users(id),
    content     TEXT NOT NULL,       -- JSON-encoded rich text segments
    delivered   BOOLEAN DEFAULT FALSE,
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Offline Message Queue

- When a message is sent to an offline user, it is stored in the `messages` table with `delivered = FALSE`.
- On login, the server queries for all undelivered messages for that user, sends them as an `OfflineMessages` batch, and marks them as `delivered = TRUE`.

### Presence Management

- Server maintains an in-memory `HashMap<UserId, Session>` of connected clients.
- When a user changes status, the server broadcasts a `PresenceUpdate` to all of that user's online contacts.
- **Invisible mode:** User appears offline to all contacts but can still send/receive messages. The server sends `PresenceUpdate` with status `Offline` to contacts when the user is actually `Invisible`.

### Concurrency

- Each WebSocket connection is handled by a dedicated tokio task.
- Shared state (sessions, presence) is managed via `Arc<RwLock<AppState>>` or `tokio::sync::broadcast` channels for fan-out.

---

## 5. Authentication

### Registration

- In-app registration: the Login window includes a "Create Account" button that reveals registration fields.
- Fields: username (unique, alphanumeric + underscores, 3–20 chars) and password (min 6 chars).
- Server hashes the password with `argon2` before storing.
- On successful registration, the user is automatically logged in.

### Login

- Username + password sent via `LoginRequest`.
- Server validates against stored argon2 hash.
- On success: returns session token, contact list, pending friend requests, and offline messages.
- On failure: returns error with reason (invalid credentials, username not found).

### Session Conflict

- If a user logs in while already logged in elsewhere, the **previous session is kicked**.
- The old client receives a `KickedNotify` message with reason "Signed in from another location" before being disconnected.
- The new client proceeds normally.

---

## 6. Multi-Window Architecture

### iced::daemon

The client uses `iced::daemon` for true multi-window support. Three window types exist:

#### Window Types

1. **Login Window**
   - Shown on app launch.
   - Username/password fields, "Sign In" button, "Create Account" toggle.
   - Closes after successful authentication.
   - Non-resizable, fixed size, centered on screen.

2. **Buddy List Window**
   - Opens after successful login.
   - Displays contacts organized in collapsible groups.
   - Shows user's own status, avatar, and username at the top.
   - Contains a toolbar/menu for status changes and settings.
   - Closing this window exits the application (or minimizes to tray if tray is enabled).
   - Single instance — only one buddy list window exists.

3. **Chat Window**
   - One per active conversation.
   - Opens on: double-clicking a contact in the buddy list, or receiving a new message from a contact with no open chat window.
   - Contains: chat history display (custom rich text widget), message input area (with formatting toolbar), and recipient info header.
   - Multiple chat windows can be open simultaneously.
   - Closing a chat window does not end the conversation — reopening shows cached history.

#### Window Management

- Each window has a unique `iced::window::Id`.
- The app's central `update()` function routes messages to the correct window based on window ID.
- Chat windows are tracked in a `HashMap<window::Id, ChatWindowState>` which maps window IDs to conversation partners.
- Window positions and sizes are not persisted across sessions.

---

## 7. Buddy List

### Layout

- **Header section:** User's avatar (from predefined gallery), display name, current status dropdown.
- **Contact groups:** Collapsible sections with group name headers. Each group shows its online/total count (e.g., "Friends (3/7)").
- **Contact entries:** Avatar thumbnail, username, status icon (colored dot), custom status message (truncated).
- Online contacts are sorted to the top within each group, then alphabetically.
- Offline contacts are greyed out.

### Groups

- Users can create, rename, and delete groups.
- Default group: "Friends" (cannot be deleted, only renamed).
- Contacts are moved between groups via drag-and-drop.
- Groups can be reordered via drag-and-drop.
- Empty groups are still displayed (can be collapsed).

### Context Menu

Right-clicking a contact shows:
- Send Message
- View Profile (shows avatar + status in a tooltip/popup)
- Move to Group → (submenu of groups)
- Remove Contact
- (separator)
- Copy Username

Right-clicking a group header shows:
- Rename Group
- Delete Group (moves contacts to "Friends")
- Create New Group

### Search

- A search/filter field at the top of the buddy list.
- Filters contacts by username as the user types.
- Groups that have no matching contacts are hidden during search.

---

## 8. Contact System

### Adding Contacts

1. User types a username into an "Add Contact" field (in buddy list toolbar or menu).
2. Client sends `AddContactRequest` with the target username.
3. Server validates the username exists and no existing relationship or pending request exists.
4. Server stores the request in `contact_requests` table.
5. If the target user is online, they receive a `ContactRequestReceived` message immediately.
6. If offline, they receive it as part of login sync.

### Receiving Requests

- Pending friend requests are shown in a notification area (badge count on buddy list, or a dedicated "Requests" section at the top).
- Each request shows: sender username, optional message, Accept/Deny buttons.
- **Accept:** Both users are added to each other's contact lists. Both receive updated contact lists. The new contact appears in the "Friends" group by default.
- **Deny:** Request is deleted. The sender is not notified of the denial (silent deny, like original Yahoo).

### Removing Contacts

- Removing a contact is one-directional: User A removes User B from their list, but User B still has User A unless they also remove.
- Server deletes the row from `contacts` for the removing user only.

---

## 9. Chat

### Message Format

Messages are stored and transmitted as an array of rich text segments:

```rust
// shared/src/message.rs
#[derive(Serialize, Deserialize)]
enum Segment {
    Text {
        content: String,
        bold: bool,
        italic: bool,
        underline: bool,
        color: Option<String>,   // hex color, e.g., "#FF0000"
        size: Option<u8>,        // font size in points
    },
    Emoticon {
        id: String,              // emoticon identifier, e.g., ":smile:"
    },
}

type RichMessage = Vec<Segment>;
```

### Formatting Toolbar

The chat input area includes a formatting toolbar with:
- **B** / **I** / **U** buttons (toggle bold, italic, underline)
- Font color picker (opens a small color palette)
- Font size selector (dropdown: 8, 10, 12, 14, 18, 24)
- Emoticon picker button (opens emoticon grid popup)

Formatting applies to newly typed text from the cursor position forward. Already-typed text can be formatted by selecting it and clicking a format button.

### Chat Display

- Messages are rendered using the **custom rich text widget** (see section 12).
- Each message shows: sender name (in their chosen color), timestamp, then the message content with inline emoticons.
- Consecutive messages from the same sender within 1 minute are grouped (no repeated name/timestamp).
- The chat scrolls to the bottom on new messages. If the user has scrolled up, a "New messages" indicator appears instead of auto-scrolling.

### Typing Indicators

- When a user types in the chat input, the client sends `TypingStart` to the server.
- Typing events are **throttled to 1 per 3 seconds** — the client tracks the last sent time and suppresses intermediate events.
- The server relays `TypingStart` to the recipient.
- The recipient's chat window shows "{username} is typing..." below the chat area.
- The indicator disappears after 5 seconds of no `TypingStart` from the sender, or when a message is received.
- When the user stops typing (input is empty or idle for 5 seconds), a `TypingStop` is sent.

---

## 10. Presence System

### Status States

| Status | Icon | Description |
|---|---|---|
| Online | Green dot | User is active |
| Away | Yellow dot | User is away (manual or auto-idle) |
| Busy | Red dot | User is busy, do not disturb |
| Invisible | — | Appears offline to others, can still chat |
| Offline | Grey dot | User is disconnected |

### Custom Status Message

- Users can set a free-text custom status message (max 100 chars) that appears next to their name in contacts' buddy lists.
- Custom messages are independent of status state — a user can be "Busy" with the message "In a meeting".

### Auto-Idle

- If no mouse/keyboard input is detected for 10 minutes, the client automatically changes status to "Away".
- When activity resumes, status reverts to the previous state.
- Auto-idle only triggers if the user's current status is "Online". It does not override "Busy" or "Invisible".

### Status Change Flow

1. User selects status from dropdown in buddy list header.
2. Client sends `StatusChange` to server.
3. Server updates user's status in memory and database.
4. Server broadcasts `PresenceUpdate` to all of that user's online contacts.
5. Contacts' buddy lists update the status icon and resort if needed.

---

## 11. Buzz / Nudge

### Sending

- Each chat window has a "Buzz!" button (or lightning bolt icon).
- Clicking it sends a `BuzzSend` message to the server.
- The server relays it as `BuzzReceived` to the recipient.
- **No rate limiting** — users can buzz as frequently as they want.

### Receiving

- When a `BuzzReceived` message arrives:
  1. **Window shake:** The chat window is physically moved on screen using OS window position APIs. The shake pattern:
     - Duration: ~1 second
     - Pattern: Rapidly alternate the window position by ±10px horizontally and ±8px vertically, ~30 times over the duration.
     - Implementation: Use `winit` or platform-specific APIs to set window position. Driven by a short-lived async task or animation tick.
  2. **Sound effect:** Play the buzz sound via `rodio`.
  3. **Visual indicator:** Display "🔔 {username} has buzzed you!" in the chat history as a system message.
  4. If no chat window is open for that sender, one is created first, then the buzz effect plays.
  5. If the app is minimized or in the tray, the tray icon flashes and the chat window is brought to front.

---

## 12. Custom Emoticon Widget

### Purpose

Iced's built-in `Text` widget does not support inline images. A custom widget is needed to render chat messages containing both styled text and emoticon images on the same line.

### Design

The `RichTextLine` custom widget:

- **Input:** A `Vec<Segment>` (text segments and emoticon references).
- **Layout:** Performs text measurement to flow segments left-to-right, wrapping to new lines as needed. Emoticons are treated as inline elements with the same height as the surrounding text.
- **Rendering:**
  - Text segments: Drawn with `iced::widget::canvas` or `iced::advanced::text` primitives, respecting bold/italic/underline/color/size.
  - Emoticon segments: Rendered as `iced::advanced::image` at the correct inline position, scaled to match line height.
- **Interaction:** Supports text selection (stretch goal — may be deferred).

### Emoticon Set

- Use an open-source emoji set (e.g., OpenMoji, Twemoji) as PNG sprites.
- Map Yahoo-style shortcodes (`:)`, `:D`, `;)`, `:P`, `:(`, `:|`, `:O`, `>:)`, `B)`, `:'(`, etc.) to specific emoji images.
- The emoticon picker displays all available emoticons in a grid popup. Clicking one inserts the shortcode into the chat input.
- Emoticons in the input field are displayed as shortcodes (text). They are only rendered as images in the chat display.

---

## 13. Avatars

### Predefined Gallery

- A set of ~20–30 built-in avatar images bundled with the client.
- Stored in `assets/avatars/` as PNG files (64x64 or 128x128).
- The gallery includes a mix of character types similar to Yahoo's default set: smiley faces, animals, abstract icons, etc.
- Sourced from open-source icon/avatar sets.

### Selection

- Users choose their avatar from the gallery via a grid picker in the buddy list header (click on current avatar to open).
- Selection sends an `AvatarChange` message to the server, which stores the avatar ID and broadcasts the change to online contacts.

### Display

- Buddy list: Small thumbnail (24x24 or 32x32) next to each contact name.
- Chat window: Medium (48x48) avatar in the header area showing who you're chatting with.
- Profile popup: Full size (64x64 or 128x128) when viewing a contact's profile.

---

## 14. Audio

### Library

`rodio` for all audio playback. Sounds are played on a dedicated audio thread (rodio handles this internally).

### Sound Events

| Event | Sound | Trigger |
|---|---|---|
| Contact online | Door open chime | A contact in your buddy list comes online |
| Contact offline | Door close sound | A contact in your buddy list goes offline |
| Message received | Short ding/chime | New chat message received (and chat window is not focused) |
| Buzz received | Buzz/vibration sound | Another user sends you a buzz |
| Login success | Welcome chime | Successful authentication |
| Error | Error beep | Failed action (login fail, etc.) |

### Asset Format

- Sounds stored as `.wav` or `.ogg` files in `assets/sounds/`.
- Sourced from creative-commons / freely-licensed sound libraries.
- Sounds should be short (< 2 seconds) and evocative of the Yahoo Messenger era without copying the original assets.

### Volume Control

- A global mute toggle in the buddy list toolbar/menu.
- No per-sound volume control (keeps UI simple).

---

## 15. System Tray

### Integration

Uses `tray-icon` crate (or platform-specific equivalent) for Windows system tray support.

### Behavior

- **Tray icon:** Appears when the app is running. Shows current status via icon color/overlay:
  - Green: Online
  - Yellow: Away
  - Red: Busy
  - Grey: Invisible / Disconnected

- **Minimize to tray:** Closing the buddy list window minimizes to tray instead of exiting. The app continues running. Double-click tray icon to restore.

- **Flash on new message:** When a message is received and no chat window is focused, the tray icon flashes/blinks to attract attention.

- **Right-click context menu:**
  - My Status → Online / Away / Busy / Invisible
  - Open Messenger (restore buddy list)
  - Mute Sounds (toggle)
  - Sign Out
  - Exit (fully close app)

---

## 16. Local Storage (Client)

### SQLite Cache

The client maintains a local SQLite database for chat history persistence:

```sql
-- Local message cache
CREATE TABLE messages (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation  TEXT NOT NULL,     -- username of the other party
    sender        TEXT NOT NULL,     -- 'me' or their username
    content       TEXT NOT NULL,     -- JSON-encoded rich text segments
    timestamp     INTEGER NOT NULL,  -- unix millis
    is_buzz       BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_messages_conv ON messages(conversation, timestamp);
```

### Behavior

- Messages are saved locally as they are sent and received.
- When a chat window opens, the client loads recent messages from the local cache (last 100 messages for that conversation).
- The local cache is the source of truth for chat history display — the server's stored messages are only used for offline delivery.
- Cache location: OS-appropriate app data directory (e.g., `%APPDATA%/yahoo-messenger/` on Windows).

---

## 17. Connection Handling

### Manual Reconnect

- When the WebSocket connection drops (server restart, network issue, etc.), the client:
  1. Stops sending/receiving messages.
  2. Shows a "Disconnected" banner at the top of the buddy list window.
  3. Greys out all contacts (unknown status).
  4. Chat windows show "Disconnected — messages cannot be sent" in the input area.
  5. The buddy list displays a "Reconnect" button.
- Clicking "Reconnect" attempts to re-establish the WebSocket connection and re-authenticate with stored credentials.
- On successful reconnect: presence is restored, contact statuses are refreshed, offline messages are delivered.
- On failed reconnect: error message shown, user can try again.

---

## 18. Visual Style

### Color Palette (Faithful to v5–6 era)

- **Primary background:** Light grey (`#E8E8E8` – `#F0F0F0`)
- **Buddy list header:** Gradient blue/purple (Yahoo signature)
- **Group headers:** Slightly darker grey with bold text
- **Online contact text:** Black
- **Offline contact text:** Grey (`#999999`)
- **Chat input area:** White
- **Chat display area:** White with subtle border
- **Toolbar/menu:** Light grey with separator lines
- **Accent/buttons:** Yahoo purple (`#7B0099`) and blue (`#0066CC`)

### Typography

- Primary font: System sans-serif (Segoe UI on Windows).
- Chat messages: User-configurable size (8–24pt) via formatting toolbar.
- Buddy list: Fixed size (~11pt for contacts, ~10pt for status messages).

### Iconography

- Status dots: Simple colored circles (green/yellow/red/grey).
- Toolbar icons: Minimal line-art style. Unicode symbols where possible (as per existing CLAUDE.md guidance).
- Window chrome: Standard OS window decorations (no custom title bar).

---

## 19. Assets

All assets are **open-source alternatives** — no copyrighted Yahoo material.

### Emoticons
- Source: OpenMoji, Twemoji, or similar open emoji set.
- Format: PNG, sized to match text line height (16–24px base).
- Count: ~30–40 core emoticons covering the most common Yahoo set (smile, grin, wink, tongue, sad, angry, cool, cry, surprised, etc.).

### Sounds
- Source: Freesound.org, OpenGameArt, or similar CC-licensed libraries.
- Format: `.wav` or `.ogg`, short clips (< 2 seconds each).
- ~6 distinct sounds (see Audio section).

### Avatars
- Source: Open-source avatar/icon sets or AI-generated (with clear licensing).
- Format: PNG, 128x128.
- Count: ~20–30 options.

---

## 20. Phased Milestones

### Phase 1: Foundation
**Goal:** Workspace structure, server skeleton, basic client with login.

- Set up Cargo workspace with `client`, `server`, `shared` crates.
- Define protocol message types in `shared`.
- Implement server: WebSocket listener, user registration, login/auth with argon2, session management, kick-previous-session.
- Implement client: Login window with `iced::daemon`, WebSocket connection, login/register flow.
- **Deliverable:** User can register, log in, and see a blank buddy list window. Server handles multiple concurrent connections.

### Phase 2: Contacts & Buddy List
**Goal:** Contact system and buddy list UI.

- Server: Contact request flow (send/accept/deny), contact list storage, group management.
- Client: Buddy list UI with groups, collapsible sections, online/offline sorting.
- Client: Add contact flow, friend request notifications, accept/deny UI.
- Client: Group management (create, rename, delete, drag-and-drop contacts between groups).
- Client: Contact search/filter.
- **Deliverable:** Two users can add each other, organize contacts into groups, and see each other's online/offline status.

### Phase 3: Chat & Messaging
**Goal:** Core chat functionality.

- Server: Message routing between online users, offline message queuing and delivery.
- Client: Chat windows (open on double-click, one per conversation).
- Client: Basic text messaging (send/receive, display in chat history).
- Client: Typing indicators.
- Client: Local SQLite cache for message history.
- Client: Message grouping and timestamps.
- **Deliverable:** Two users can chat in real-time with typing indicators. Messages are cached locally and offline messages are delivered on login.

### Phase 4: Presence & Status
**Goal:** Full presence system.

- Server: Status tracking, broadcasting status changes, invisible mode logic.
- Client: Status dropdown in buddy list header (Online, Away, Busy, Invisible).
- Client: Custom status message field.
- Client: Auto-idle detection (10-minute timeout).
- Client: Status icons in buddy list, online sorting.
- Client: Door open/close presence sounds.
- **Deliverable:** Full presence lifecycle — status changes propagate to contacts, invisible mode works, auto-idle triggers.

### Phase 5: Rich Features
**Goal:** The signature Yahoo Messenger experience.

- Client: Custom rich text widget for inline emoticons.
- Client: Formatting toolbar (bold, italic, underline, color, size).
- Client: Emoticon picker and shortcode parsing.
- Client: Buzz/nudge with OS window shake effect.
- Client: All sound effects via rodio.
- Client: Predefined avatar gallery and selection.
- Server: Avatar change broadcasting.
- **Deliverable:** Full rich text chat with emoticons, buzz effect, sounds, and avatars.

### Phase 6: Polish
**Goal:** System tray, disconnection handling, visual refinement.

- Client: System tray integration (minimize to tray, flash on messages, context menu).
- Client: Disconnected state UI and manual reconnect.
- Client: Visual polish — colors, spacing, typography matching the v5–6 aesthetic.
- Client: Context menus on buddy list (right-click contacts and groups).
- Client: Mute sounds toggle.
- Client: Window management edge cases (buzz on minimized windows, tray restore).
- **Deliverable:** Production-quality application with all features integrated and polished.
