# Admin API Specification (Tauri Commands)

This document describes the IPC commands exposed by the Tauri backend for the Admin Client.
These commands are invoked using `invoke('command_name', { args })` from the frontend.

## 1. Server Control

### `start_server`
Starts the embedded HTTP server using the port defined in the configuration.

- **Request**:
  - Arguments: None (Uses `Settings.server_port`)
- **Response**:
  - Success: `null` (void)
  - Error: `String` (Error message)

### `stop_server`
Stops the embedded HTTP server.

- **Request**:
  - Arguments: None
- **Response**:
  - Success: `null` (void)
  - Error: `String` (Error message)

### `is_server_running`
Checks if the embedded HTTP server is currently running.

- **Request**:
  - Arguments: None
- **Response**:
  - Success: `boolean`
  - Error: `String` (Error message)

### `reload_settings`
Reloads `settings.toml` without restarting the application.

- **Request**:
  - Arguments: None
- **Response**:
  - Success: `ReloadSettingsResponse`
    ```ts
    {
      settings: {
        server_port: number;
        db_path: string;
        wifi_ssid?: string | null;
        wifi_password?: string | null;
        wifi_encryption?: string | null;
      };
      changed: {
        server_port: boolean;
        db_path: boolean;
        wifi: boolean;
      };
    }
    ```
  - Error: `String` (Error message)

---

## 2. Event Management

### `admin_create_event`
Creates a new lottery event.

- **Request**:
  - `title` (string): The name of the event.
- **Response**:
  - Success: `Event` object
    ```ts
    {
      id: string;         // UUID
      title: string;
      status: "Active" | "Closed" | "Deleted";
      created_at: number; // Unix timestamp
      updated_at: number; // Unix timestamp
    }
    ```
  - Error: `String` (Error message, e.g., "Title cannot be empty")

### `admin_list_events`
Retrieves a list of all events (excluding physically deleted ones, though soft-deleted might be included depending on implementation, currently logic filters out "Deleted" status for public list, but admin might see all).
*Note: Current implementation in `logic.rs` filters out `EventStatus::Deleted`.*

- **Request**:
  - Arguments: None
- **Response**:
  - Success: `Event[]` (Array of Event objects)
  - Error: `String`

### `admin_update_event_status`
Updates the status of an existing event.

- **Request**:
  - `event_id` (string): UUID of the event.
  - `status` (string): Target status. One of `"Active"`, `"Closed"`, `"Deleted"`.
    *(Note: Frontend types usually map "Ended" to "Closed")*
- **Response**:
  - Success: `null` (void)
  - Error: `String` (e.g., "Event not found")

### `admin_delete_event`
Soft-deletes an event (sets status to "Deleted").

- **Request**:
  - `event_id` (string): UUID of the event.
- **Response**:
  - Success: `null` (void)
  - Error: `String`

---

## 3. Comments & Lottery

### `admin_get_comments`
Retrieves all comments for a specific event.

- **Request**:
  - `event_id` (string): UUID of the event.
- **Response**:
  - Success: `Comment[]` (Array of Comment objects)
    ```ts
    {
      id: string;          // UUID
      event_id: string;    // UUID
      nickname: string;
      content: string;
      phone_masked: string; // e.g., "138****1234" (Note: Admin API currently returns masked phone too)
      is_winner: boolean;
      created_at: number;
    }
    ```
  - Error: `String`

### `admin_draw_winner`
Randomly selects a winner from the comments of an active event.
**Side Effect**: Automatically sets the event status to `Closed`.

- **Request**:
  - `event_id` (string): UUID of the event.
- **Response**:
  - Success: `Comment` object (The winning comment)
  - Error: `String` (e.g., "Event not found", "No comments available", "Event is closed")
