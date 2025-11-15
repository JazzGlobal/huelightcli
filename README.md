# 🗺️ Project Roadmap

This roadmap outlines the development plan for the HueLight backend, including the core library and command-line interface. Tasks are intentionally structured to support future expansion into a web API and Angular frontend.

---

## 🚀 Milestone 1 — Project Structure & Core Library

**Goal:** Establish a clean multi-crate workspace and centralize all Hue interaction logic.

- [X] Create workspace structure:  
  - [X] `huelight-core/` (library crate)
  - [X] `huelight-cli/` (binary crate)
- [X] Implement `huelight-core` foundation:  
  - [X] `client.rs` (HueClient trait + ReqwestHueClient)  
  - [ ] (Optional) `MockHueClient`  
  - [X] `models.rs` (Light, LightState, DTOs)  
  - [X] `config.rs` (load/save config)  
  - [X] `hue_api.rs` (high-level API wrapper)  
  - [X] `lib.rs` exporting all modules

---

## 💡 Milestone 2 — Basic Hue Integration (Core Functionality)

**Goal:** End-to-end communication with the Hue Bridge using real config, pairing, and parsed models.

- [X] Configuration system:  
  - [X] Choose config file location  
  - [X] Implement load/save/validate  
- [ ] Bridge pairing / setup workflow:  
  - [ ] Implement `register_user(bridge_ip)`  
  - [ ] Handle “press link button” gracefully  
  - [X] CLI `setup` command to save config  
- [X] Core Hue operations via `HueApi`:  
  - [X] `get_lights()`  
  - [X] `set_light_state(id, LightStateUpdate)`  
  - [X] Raw Hue JSON → domain model mapping  

---

## 🛠️ Milestone 3 — Usable CLI (Daily Driver)

**Goal:** Build a practical and intuitive CLI using `clap`, powered by `huelight-core`.

- [ ] Implement CLI commands:  
  - [X] `lights list`  
  - [X] `lights on <id>`  
  - [X] `lights off <id>`  
  - [X] `lights toggle <id>`  
  - [ ] `lights brightness <id> <0-100>`  
- [X] Connect CLI commands to core library  
- [ ] Improve UX with human-friendly output  
- [ ] Provide helpful error messages:  
  - [X] Missing config  
  - [X] Invalid IDs  
  - [ ] Bridge unreachable  

---

## 🧱 Milestone 4 — Domain Cleanup & Error Handling

**Goal:** Stabilize the backend architecture so it is clean, reusable, and API-ready.

- [ ] Refine domain model structures  
- [ ] Separate domain vs. Hue JSON DTOs  
- [ ] Introduce unified error handling via `thiserror`:  
  - [ ] `CoreError` (network, auth, config, not found, etc.)  
- [ ] Enhance `ReqwestHueClient`:  
  - [ ] Better timeouts  
  - [ ] Optional retries  
  - [ ] Cleaner propagation of Hue errors  

---

## 🧪 Milestone 5 — Testing & Mocks

**Goal:** Achieve reliability and regression protection with real tests.

- [X] Implement `MockHueClient` for isolated tests  
- [ ] Add unit tests for:  
  - [X] JSON parsing  
  - [ ] Config load/save using temp directory  
  - [ ] `HueApi` logic (using mock client)  
- [ ] (Optional) Add integration tests:  
  - [ ] With a local stub server, or  
  - [ ] Against a real Hue bridge (feature flag)  

---

## 🌐 Milestone 6 — Prepare for Rust Web API & Angular Frontend

**Goal:** Ensure the backend is decoupled, consistent, and ready for a future web UI.

- [ ] Export serializable DTOs for HTTP API use  
- [ ] Verify `huelight-core` has no CLI specifics (pure library)  
- [ ] (Optional) Scaffold `huelight-api/` crate:
  - [ ] Add first route: `GET /api/lights` (Axum + HueApi)  
- [ ] Confirm architecture cleanly supports multiple clients (CLI + Web)

---

## 🏁 Result

Upon completing this roadmap, you will have:

- A clean Rust workspace  
- A reusable core library with domain logic  
- A production-ready CLI tool  
- Strong error handling & test coverage  
- A backend fully prepared for a web API  
- A perfect foundation for a modern Angular frontend
