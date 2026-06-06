# Phase 2 Role Implementation Guides

This directory contains role-specific implementation guides for Phase 2 user-system work. These files do not replace the source documents; they make the existing plan easier to split among multiple people or agents inside the same role.

## Source Of Truth

Read these first, in order:

1. `README.md`
2. `agent/AGENT_RULES.md`
3. `agent/COLLABORATION_PLAN.md`
4. The role guide that matches your assigned work

If this directory disagrees with `README.md` or `agent/COLLABORATION_PLAN.md`, stop and update the role guide before implementation. Do not expand product scope beyond the README Phase 2 checklist.

## Role Documents

| Role | Guide | Scope |
|---|---|---|
| A | `A_BACKEND_API_AUTH.md` | Rust backend, API routes, auth, backend tests, backend dependencies |
| B | `B_FRONTEND_PAGE_INTERACTION.md` | Vue pages, components, router, API client, form interaction, frontend build |
| C | `C_DATABASE_CONTRACTS_DOCS_QA.md` | Database structure, contracts, field mapping, env docs, README status, integration QA |

## Phase 2 Scope Guardrails

Allowed scope is only the README Phase 2 user system:

- User model and table planning.
- Register/login API with JWT and bcrypt.
- Auth middleware.
- Current user and profile update APIs.
- Password change API.
- Avatar upload API only after storage strategy is confirmed.
- Register/login pages, logout, profile page, and Phase 2-only favorites placeholder UI.

Do not start Phase 3 game browsing, Phase 4 comments/likes/favorites data, Phase 5 admin/resources, or Phase 6 search/deploy work from these role guides.

## Execution Model

Use the existing contract-first wave order:

1. Role C locks database, API, field, and env contracts.
2. Role A implements backend from those contracts.
3. Role B implements frontend from those contracts.
4. A/B/C perform integration checks together.

If one role has multiple people, split by sub-lane inside that role. Each sub-lane must have one owner, one handoff note, and explicit shared-file impact. Do not let two people edit the same high-conflict file without sequencing.

## Splitting One Role Across Multiple People

Each role guide defines sub-lanes. Use these rules:

- Assign exactly one owner to each atomic implementation point.
- Keep one atomic point small enough to complete with one focused implementation and one verification pass.
- A sub-owner may prepare tests or docs while another sub-owner waits for a contract, but cannot mark the feature complete until the dependent contract exists.
- Shared files must be edited in the order defined by `agent/COLLABORATION_PLAN.md` section 4.5.
- Every sub-owner leaves a handoff note with changed files, contracts consumed or changed, verification, known limits, and conflict notes.

## Verification-First Rule

Before writing implementation code, each atomic point must define:

- Happy path check.
- Edge or failure check.
- Existing regression check.
- Real surface to verify: test command, curl, build, or browser/manual route check.

For code tasks, write the test or expected verification scenario before implementation. For docs-only tasks, define the acceptance check before editing.

## Stop Conditions

Stop and ask the role lead or project owner if any of these are true:

- README and actual code state disagree.
- The API response shape is unclear.
- A DB/Rust/API/frontend field has more than one name.
- Migration tool or directory is undecided but a task requires a table.
- Token storage strategy is undecided but a task requires auth state.
- Avatar storage strategy is undecided but a task requires upload implementation.
- Two people need to modify the same high-conflict file without an agreed order.
- Verification cannot run and no alternative evidence is available.

## Docs-Only Verification

After editing role guides, verify:

```bash
rg -n "A-0[1-7]|B-0[1-7]|C-0[1-4]|INT-0[1-3]" agent/roles
rg -n "Phase 3|Phase 4|Phase 5|Phase 6" agent/roles
git diff --check
npm run lint
```

`Phase 3+` mentions are allowed only as explicit scope guards saying not to implement them from these guides.
