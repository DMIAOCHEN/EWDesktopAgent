---

description: "Task list for intelligent desktop client implementation"
---

# Tasks: 智能桌面客户端

**Input**: Design documents from `/specs/001-intelligent-desktop-client/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Not explicitly requested in spec - following Constitution II (Test-First) approach, tests should be created for core modules

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Tauri Client**: `src-tauri/` for Rust code, `src/` for renderer (TypeScript)
- **AI Gateway**: `ai-gateway/` (ASP.NET Core solution)
- **Tests**: `src-tauri/tests/`, `src/tests/`, `ai-gateway/tests/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Initialize Tauri 2.x project with TypeScript frontend
- [x] T002 [P] Configure Rust toolchain and dependencies in Cargo.toml
- [x] T003 [P] Configure TypeScript/Vite frontend build
- [x] T004 Setup logging system (tracing for Rust, console for frontend)
- [x] T005 [P] Configure SQLite database withrusqlite
- [x] T006 Create .NET Core 8.0 solution for AI Gateway
- [x] T007 [P] Setup PostgreSQL connection in Gateway
- [x] T008 Configure CI/CD pipeline structure

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**CRITICAL**: No user story work can begin until this phase is complete

- [x] T009 [P] Implement WebView2 browser wrapper in Rust
- [x] T010 [P] Create system tray and window management
- [x] T011 [P] Implement Tauri IPC communication layer
- [x] T012 [P] Create SQLite database schema with all entities
- [x] T013 [P] Implement user authentication flow (login/logout)
- [x] T014 [P] Implement business system configuration loader
- [x] T015 [P] Create FastGPT API client service
- [x] T016 [P] Setup AI Gateway basic API structure (.NET Core)
- [x] T017 [P] Implement Token mapping service in Gateway
- [x] T018 Create base UI components (tabs, navigation, settings)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - 浏览器基础功能 (Priority: P1) 🎯 MVP

**Goal**: Enable users to access pre-configured business systems (RIS/PIS/EIS) through embedded browser

**Independent Test**: Launch client, select business system from list, page loads correctly, multi-tab works, download works

### Implementation for User Story 1

- [x] T019 [P] [US1] Create business system list UI in src/components/SystemList.tsx
- [x] T020 [P] [US1] Implement browser tab manager in src-tauri/src/browser/tab_manager.rs
- [x] T021 [US1] Implement WebView2 page loading and navigation
- [x] T022 [US1] Create browser toolbar component (back/forward/refresh)
- [x] T023 [P] [US1] Implement multi-tab session management
- [x] T024 [US1] Add file download handler and save dialog
- [ ] T025 [US1] Implement tab memory limit enforcement (<200MB per tab)

**Checkpoint**: Browser core functional - users can open and use business systems

---

## Phase 4: User Story 2 - 智能网页助手 (Priority: P1) 🎯 MVP

**Goal**: AI-powered assistant that understands user intent and executes web operations

**Independent Test**: Open assistant, give command like "search patient X", assistant executes action on current page

### Implementation for User Story 2

- [x] T026 [P] [US2] Create assistant chat UI component
- [x] T027 [US2] Implement assistant command parser
- [ ] T028 [P] [US2] Create WebView2 DOM injection for operation execution
- [ ] T029 [US2] Implement action executor (click, input, navigate, search)
- [ ] T030 [P] [US2] Create page context extractor (current URL, forms, links)
- [x] T031 [US2] Integrate FastGPT client for intent understanding
- [x] T032 [US2] Implement action result display and feedback

**Checkpoint**: Assistant can understand and execute web operations

---

## Phase 5: User Story 7 - 操作风险分级与管控 (Priority: P1) 🎯 MVP

**Goal**: Risk assessment and user confirmation for sensitive operations

**Independent Test**: Send high-risk command, system prompts for confirmation, medium-risk shows preview

### Implementation for User Story 7

- [x] T033 [P] [US7] Define risk rules configuration in src/core/security/risk_rules.json
- [x] T034 [US7] Implement risk assessment engine
- [x] T035 [P] [US7] Create confirmation dialog UI component
- [x] T036 [US7] Implement operation preview display
- [x] T037 [US7] Add whitelist management UI
- [x] T038 [US7] Implement operation audit logging

**Checkpoint**: Risk control functional for all assistant operations

---

## Phase 6: User Story 3 - 智能提醒 (Priority: P2)

**Goal**: Intelligent notifications based on user-configured rules

**Independent Test**: Create reminder rule, trigger condition met, notification appears, click navigates to target

### Implementation for User Story 3

- [x] T039 [P] [US3] Create reminder rule configuration UI
- [x] T040 [US3] Implement reminder rule engine
- [x] T041 [P] [US3] Create system notification service (Windows Toast)
- [x] T042 [US3] Implement notification click handler (navigate to target)
- [x] T043 [US3] Add offline caching for reminder records
- [ ] T044 [US3] Implement sync on reconnection

**Checkpoint**: Reminders work online and sync when offline

---

## Phase 7: User Story 4 - 语音交互 (Priority: P2)

**Goal**: Voice input and output for hands-free operation

**Independent Test**: Press voice button, speak command, system recognizes and executes, voice feedback plays

### Implementation for User Story 4

- [ ] T045 [P] [US4] Integrate Snowboy wake word engine
- [ ] T046 [P] [US4] Integrate Silero VAD for voice activity detection
- [x] T047 [US4] Create audio capture service
- [ ] T048 [P] [US4] Implement ASR client (third-party API integration)
- [ ] T049 [P] [US4] Integrate Kokoro TTS engine
- [x] T050 [US4] Implement TTS fallback to cloud service
- [x] T051 [US4] Create voice activation button UI
- [x] T052 [US4] Implement voice feedback playback

**Checkpoint**: Voice interaction fully functional

---

## Phase 8: User Story 5 - AI对话助手 (Priority: P3)

**Goal**: Natural language conversation with AI for business queries

**Independent Test**: Ask AI about business data, receive intelligent response

### Implementation for User Story 5

- [x] T053 [P] [US5] Create chat history storage
- [x] T054 [US5] Implement FastGPT conversation API integration
- [x] T055 [P] [US5] Create chat UI with message history
- [x] T056 [US5] Implement business context injection
- [ ] T057 [US5] Add streaming response support

**Checkpoint**: AI chat fully functional

---

## Phase 9: User Story 6 - 个性化学习与智能推荐 (Priority: P2)

**Goal**: Learn user habits and provide personalized recommendations

**Independent Test**: Use client for 2 weeks, system remembers habits, provides proactive recommendations

### Implementation for User Story 6

- [x] T058 [P] [US6] Implement user behavior logger
- [x] T059 [US6] Create behavior pattern analysis engine
- [x] T060 [P] [US6] Implement time pattern learning
- [x] T061 [US6] Create recommendation engine
- [x] T062 [P] [US6] Implement recommendation feedback loop
- [x] T063 [US6] Add user preference management UI

**Checkpoint**: Personalization learns and adapts

---

## Phase 10: User Story 8 - 本地文件系统操作 (Priority: P2)

**Goal**: File system operations like organizing folders

**Independent Test**: Ask to organize folder, system shows plan, user confirms, files organized

### Implementation for User Story 8

- [x] T064 [P] [US8] Implement file system access layer (Rust)
- [x] T065 [US8] Create file operation permission manager
- [x] T066 [P] [US8] Implement file organizer (by date/type/name)
- [x] T067 [US8] Add operation preview generation
- [x] T068 [US8] Create file operation UI dialogs

**Checkpoint**: File operations work safely

---

## Phase 11: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T069 [P] Memory optimization pass (target <500MB total)
- [ ] T070 [P] Performance optimization (startup <5s)
- [ ] T071 [P] Security audit and penetration testing
- [ ] T072 [P] Accessibility improvements
- [ ] T073 [P] Localization support preparation
- [ ] T074 Update quickstart.md with final workflows
- [ ] T075 [P] Create user documentation
- [ ] T076 [P] Performance testing and reporting
- [ ] T077 [P] Final integration testing

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - US1, US2, US7 are P1 - implement in order
  - US3, US4, US6, US8 are P2 - can proceed in parallel after P1
  - US3, US4, US6 can run in parallel
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### Within Each User Story

- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- T019, T020, T026, T033, T039, T045, T053, T058, T064 can start after Foundational
- Within phases, parallel tasks marked [P]

---

## Implementation Strategy

### MVP First (US1 + US2 + US7)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 - Browser
4. Complete Phase 4: User Story 2 - Assistant
5. Complete Phase 5: User Story 7 - Risk Control
6. **STOP and VALIDATE**: Test MVP independently
7. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add US1 → Browser functional → Deploy/Demo
3. Add US2 + US7 → AI Assistant with safety → Deploy/Demo (MVP!)
4. Add P2 features (US3, US4, US6, US8) → Full product
5. Polish → Production release

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: US1 (Browser)
   - Developer B: US2 + US7 (Assistant + Risk)
   - Developer C: US4 (Voice) - longest pole
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Per Constitution II: Tests should be written for core modules
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

## Tech Stack Summary

| Component | Technology |
|-----------|------------|
| Client Framework | Tauri 2.x (Rust) |
| Frontend | TypeScript + React |
| Browser Engine | WebView2 |
| Local Storage | SQLite |
| AI Gateway | .NET Core 8.0 |
| Database (Gateway) | PostgreSQL |
| Voice Wake | Snowboy + Silero VAD |
| Voice ASR | Third-party API |
| Voice TTS | Kokoro + Cloud fallback |
