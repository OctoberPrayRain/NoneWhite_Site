# Role C Database/Contracts/Docs/QA Implementation Guide

Role C owns contract-first coordination for Phase 2 user-system work. This guide splits database, contract, documentation, and QA work into small implementation points so multiple Role C people or agents can coordinate A and B without creating conflicting sources of truth.

## Required Reading

Read these before editing contracts, schema, or docs:

1. `README.md`
2. `agent/AGENT_RULES.md`
3. `agent/COLLABORATION_PLAN.md`
4. `agent/roles/README.md`
5. Existing env examples, Docker config, and any database/API docs that already exist

Role C must not mark README tasks complete unless implementation exists and verification evidence is recorded.

## Ownership And Shared Files

Role C owns contract and coordination surfaces:

```txt
README.md
agent/AGENT_RULES.md
agent/COLLABORATION_PLAN.md
agent/roles/**
.env.example
docker-compose.yml
server/migrations/      # if SQL migrations are chosen
server/seeds/           # if dev seeds are added
docs/api/               # if API contracts are split out
docs/database/          # if DB docs are split out
```

High-conflict files require explicit handoff:

```txt
README.md
agent/AGENT_RULES.md
agent/COLLABORATION_PLAN.md
.env.example
server/.env.example
docker-compose.yml
server/src/routes/mod.rs
client/src/router/index.js
client/src/api/http.js
server/Cargo.toml
client/package.json
```

Role C may review A/B implementation for contract alignment, but must not rewrite A/B code without agreed ownership transfer.

## Contract-First Rule

Role C locks contracts before Role A and Role B implement against them:

1. Confirm fields and naming across DB, Rust, API JSON, and frontend variables.
2. Confirm endpoint method, path, auth requirement, request DTO, response DTO, success HTTP status, and failure codes.
3. Confirm env variable names, examples, and precedence.
4. Confirm migration location or tool.
5. Record verification commands before implementation begins.

If a contract changes after A/B start, Role C must publish the change and name the affected files.

## Intra-Role Split Lanes

| Lane | Owner Scope | Related Tasks | Handoff Target |
|---|---|---|---|
| C1 Structure/Schema | migration decision, users table schema, database docs | C-01, C-02 | A1, A3 |
| C2 API Contract/Field Map | API endpoint specs, DTO shape, error codes, field mapping | C-01, C-04 | A2, A4, A5, B2, B4 |
| C3 Env/Docs/README | env examples, README status, local startup docs | C-03 | A1, A4, A5 |
| C4 Integration QA | curl cases, page/API evidence, regression checks, final handoff | C-04, INT-01, INT-02, INT-03 | All roles |

Each lane owner must say whether their output is a contract, an example, or verification evidence.

## Task Map

| Task | Purpose | Depends On | Primary Lane |
|---|---|---|---|
| C-01 | Lock database and contract file structure | README Phase 2 | C1, C2 |
| C-02 | Create users table migration/schema | C-01 | C1 |
| C-03 | Complete env variable contract | A dependency choices | C3 |
| C-04 | Maintain integration curl and API examples | A/B implemented endpoints/pages | C2, C4 |
| INT-01 | Verify register end-to-end | A-04, B-06, C-02 | C4 |
| INT-02 | Verify login and current user end-to-end | A-05, A-06, B-05, B-07 | C4 |
| INT-03 | Verify regression surfaces | Any Phase 2 integration | C4 |

## Atomic Implementation Points

### C-01 Database And Contract File Structure

| Point | Owner Output | Verification |
|---|---|---|
| C-01.1 | Decide SQL migration directory or Rust migration tool | README and plan mention one approach only |
| C-01.2 | Decide whether API contract is kept in plan or split to `docs/api/phase2-auth.md` | Only one detailed source of truth exists |
| C-01.3 | Decide where database docs live | Avoid duplicate `server/docs/database.md` and `docs/database/phase2-users.md` |
| C-01.4 | Publish affected file list for A/B | A and B can name the contract file they consume |

### C-02 Users Table Migration/Schema

| Point | Owner Output | Verification |
|---|---|---|
| C-02.1 | Create users schema/migration using chosen structure | Fields match section 9.1 and 13.2 of collaboration plan |
| C-02.2 | Add unique constraints for username and email | SQL/migration includes constraints or explicit rationale |
| C-02.3 | Add password and avatar fields with correct nullability | `password_hash` required; `avatar_url` nullable |
| C-02.4 | Define rollback or rollback limitation | Migration docs state rollback behavior |

### C-03 Env Contract

| Point | Owner Output | Verification |
|---|---|---|
| C-03.1 | Add required auth/database env variables when implementation needs them | `.env.example` and `server/.env.example` stay consistent |
| C-03.2 | Document `server/.env` vs root `.env` precedence | A can implement config without guessing |
| C-03.3 | Ensure examples contain no real secret | Secret scan only finds placeholders |
| C-03.4 | Update README local startup notes if new required env exists | New developer can start backend with documented values |

### C-04 Integration Examples And QA Notes

| Point | Owner Output | Verification |
|---|---|---|
| C-04.1 | Maintain curl examples for register, login, and me | Each example states expected HTTP status and `code` |
| C-04.2 | Maintain failure examples | Covers invalid email, wrong password, missing token |
| C-04.3 | Compare A response fields and B consumed fields | No drift between `avatar_url`, Rust field, `avatarUrl`, frontend variable |
| C-04.4 | Record verification evidence in handoff | Commands and key output are present |

### INT-01 Register Closure

| Point | Owner Output | Verification |
|---|---|---|
| INT-01.1 | Start PostgreSQL and apply migration | DB has users table |
| INT-01.2 | Register through real page or curl | API returns `code=0` and user data |
| INT-01.3 | Confirm DB row exists | User exists without plaintext password |
| INT-01.4 | Confirm response excludes secrets | No `password` or `password_hash` in response |

### INT-02 Login And Current User Closure

| Point | Owner Output | Verification |
|---|---|---|
| INT-02.1 | Login with registered user | Response has `token`, `tokenType`, `expiresIn`, `user` |
| INT-02.2 | Persist token according to contract | Refresh keeps auth state if localStorage strategy is active |
| INT-02.3 | Access `/profile` or `GET /api/users/me` | Same user is returned/displayed |
| INT-02.4 | Test invalid token | UI or API shows unauthenticated/error state |

### INT-03 Regression Checks

| Point | Owner Output | Verification |
|---|---|---|
| INT-03.1 | Verify homepage | `/` still works |
| INT-03.2 | Verify API test page | `/test-api` still displays backend result |
| INT-03.3 | Verify backend test endpoint | `GET /api/test` returns `status: ok` |
| INT-03.4 | Run project verification | `npm run lint` or documented split checks pass |

## Database Verification

For Docker or database config changes:

```bash
docker compose config
```

After migration tooling is selected, document exact commands for:

```txt
apply migration command
rollback migration command or rollback limitation
seed command, if any
```

Schema verification must check field names, nullability, unique constraints, and indexes against the role contract.

## Contract Verification

For each endpoint, verify:

- Method and path.
- Auth requirement.
- Request DTO fields.
- Response DTO fields.
- Success HTTP status.
- Success `data` shape.
- Failure HTTP status, `code`, `data`, and `message`.
- No password or secret fields in response.
- API JSON is camelCase where contracted.

## README Status Rules

Do not mark a Phase checkbox `[x]` unless all are true:

- Code or docs for that item exist.
- Required verification ran.
- Integration evidence exists if the item claims front/back integration.
- The README wording does not imply a later phase is done.

Allowed wording for partial progress:

```txt
已准备 / 待联调
后端已实现 / 前端待接入
UI 占位 / 数据接口 Phase 4 接入
```

## QA Evidence Template

```md
## QA Evidence - Role C

Task IDs:
Scenario:
Surface:
Command Or Manual Steps:
Expected:
Actual:
Result:
Artifacts:
Follow-up Needed:
```

## Handoff Checklist

```md
## Handoff - Role C

Sub-lane:
Task IDs:
Changed Files:
Contracts Created:
Contracts Changed:
Verification:
Manual/API/UI QA:
Known Limits:
Conflict Notes:
Next Role Needed:
```

## Stop Conditions

Stop before publishing contracts or marking completion if:

- Migration tool or directory is undecided.
- There are two competing API contract locations.
- Endpoint `data` shape is unclear.
- Token storage strategy is missing.
- Avatar storage strategy is missing but upload is requested.
- README status would become more complete than actual code.
- A/B changed contract-sensitive files without handoff.
