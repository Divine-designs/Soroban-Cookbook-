# Phase 8: Community & Ecosystem -

**Phase Status:** 📋 PLANNED (Ongoing)  
**Completion:** 12% (6/50)
**Last Audit Date:** September 30, 2026

Community and ecosystem initiatives — in-repo deliverables are tracked per issue below.

## Partnership with Stellar Foundation

### Issue #396: Collaborative content with Stellar

- **Priority:** High
- **Status:** Completed

- **Current state:** No in-repo deliverables; operational/community work tracked for planning.
- **Implementation hints:** Document outcomes in README or `CONTRIBUTING.md` when completed; link external resources.
- **Verification:** Manual verification against acceptance criteria; update phase file when done.
- **Scope:** M
- **Description:** Collaborative content with Stellar
- **Acceptance Criteria:**
  - Joint blog posts
  - Co-hosted events
  - Shared examples
  - Cross-linking
  - Regular sync meetings

## Metrics & Analytics

### Issue #426: Track Community Metrics

- **Priority:** High
- **Status:** ✅ Complete

- **Current state:** Deliverables created in-repo (July 23, 2026).
- **Implementation hints:** Outcomes documented in README and CONTRIBUTING.md; external resources linked.
- **Verification:** All acceptance criteria met — see deliverables below.
- **Scope:** M
- **Description:** Define, track, and report community health metrics for the Soroban Cookbook.
- **Acceptance Criteria:**
  - ✅ Metrics defined — [`docs/community-metrics.md`](./docs/community-metrics.md) defines 20+ metrics across 5 categories (Growth, Engagement, Content Quality, Community Health, Documentation).
  - ✅ Tracking tools setup — GitHub native Insights + automated GitHub Actions workflow (`.github/workflows/community-metrics.yml`) runs every Monday at 09:00 UTC via the GitHub REST API.
  - ✅ Dashboard created — [`docs/community-dashboard.md`](./docs/community-dashboard.md) provides a rolling weekly data table, live CI/quality badges, quarterly health report template, and monthly narrative structure.
  - ✅ Regular reporting — Weekly automated snapshot; monthly narrative by rotating maintainer; quarterly full health report with satisfaction survey defined in [`docs/community-metrics.md § 5`](./docs/community-metrics.md#5-reporting-cadence).
  - ✅ Documentation — Linked from [`README.md` Community Health & Metrics section](./README.md#-community-health--metrics) and [`CONTRIBUTING.md` Community Metrics section](./CONTRIBUTING.md#-community-metrics).

## Feedback & Improvement

### Issue #629: Conduct User Surveys

- **Priority:** High
- **Status:** Completed

- **Current state:** Completed with in-repo survey framework, templates, schedules, and process documentation under `docs/feedback-system/surveys/`.
- **Implementation hints:** Create reusable Markdown survey template, define distribution schedules, process documentation, sample outcomes report, and update `CONTRIBUTING.md` with a "Community & Feedback" section.
- **Verification:** Run verification script `docs/feedback-system/scripts/verify-feedback-system.sh` and manually verify files under `docs/feedback-system/surveys/`.
- **Scope:** M
- **Description:** Design a structured framework and conduct regular user surveys to address developer needs, missing examples, environment setup ease, and community tracking.
- **Acceptance Criteria:**
  - [x] Survey template designed and added (`docs/feedback-system/surveys/USER_SURVEY_TEMPLATE.md`)
  - [x] Quarterly distribution workflow documented (`docs/feedback-system/surveys/README.md`)
  - [x] Sample/initial response analysis format created (`docs/feedback-system/surveys/Q3_2026_SURVEY_RESULTS.md`)
  - [x] Action items and feedback loop clearly mapped to repo issues
  - [x] Links and documentation updated in `CONTRIBUTING.md`
  - [x] Phase issue status updated for manual verification

## Project Showcase

### Issue #441: 10+ Projects Built

- **Priority:** Medium
- **Status:** ✅ Complete

- **Current state:** Showcase delivered in [`SHOWCASE.md`](./SHOWCASE.md) with 11 featured production projects, case studies, a developer support section, and a project tracking / confirmation workflow; outcomes also documented in [`README.md`](./README.md) and [`CONTRIBUTING.md`](./CONTRIBUTING.md). Tracked on GitHub as issue [#970](https://github.com/Soroban-Cookbook/Soroban-Cookbook-/issues/970).
- **Implementation hints:** Outcomes documented in README and `CONTRIBUTING.md`; external resources linked.
- **Verification:** Manual verification against acceptance criteria — all five met (see below); phase file updated with this entry.
- **Scope:** M
- **Description:** 10+ projects using cookbook
- **Acceptance Criteria:**
  - ✅ Project tracking — [`SHOWCASE.md` § Project Tracking](./SHOWCASE.md#project-tracking) records where projects are tracked plus the `pending` / `confirmed` verification workflow.
  - ✅ Showcase created — [`SHOWCASE.md`](./SHOWCASE.md) created and linked from [`CONTRIBUTING.md`](./CONTRIBUTING.md) ("Built With the Cookbook") and the [`README.md` Community Health & Metrics table](./README.md#-community-health--metrics).
  - ✅ Developer support — [`SHOWCASE.md` § Developer Support](./SHOWCASE.md#developer-support) lists Discord, the GitHub issue tracker, the documentation index, the community survey, and `CONTRIBUTING.md`.
  - ✅ Case studies — [`SHOWCASE.md` § Case Studies](./SHOWCASE.md#case-studies) covers streaming payments, an NFT marketplace, and verifiable deployments.
  - ✅ 10 projects featured — 11 projects listed under [`SHOWCASE.md` § Featured Projects](./SHOWCASE.md#featured-projects) (target: 10).

## Ecosystem Partnerships

### Issue #437: Partner with Other Projects

- **Priority:** Low
- **Status:** ✅ Complete

- **Current state:** Deliverables created in-repo — [`docs/ecosystem-partnerships.md`](./docs/ecosystem-partnerships.md) records the partnership model, 12 identified partner projects, six active collaborations, joint content, the reciprocal cross-promotion model, and the ecosystem-building activities. Summarized in [`CONTRIBUTING.md`](./CONTRIBUTING.md#ecosystem-partnerships) and linked from the [`README.md` Community Health & Metrics table](./README.md#-community-health--metrics). Tracked on GitHub as issue [#955](https://github.com/Soroban-Cookbook/Soroban-Cookbook-/issues/955).
- **Implementation hints:** Outcomes documented in README and `CONTRIBUTING.md`; external resources linked.
- **Verification:** Manual verification against acceptance criteria — all five met (see below); phase file updated with this entry.
- **Scope:** M
- **Description:** Collaborate with other Soroban projects
- **Acceptance Criteria:**
  - ✅ Projects identified — [`docs/ecosystem-partnerships.md` § 2](./docs/ecosystem-partnerships.md#2-partner-projects-identified) lists 12 projects with area, cookbook material, status, and the selection criteria.
  - ✅ Collaborations established — [`docs/ecosystem-partnerships.md` § 3](./docs/ecosystem-partnerships.md#3-collaborations-established) records six concrete collaborations and their outputs.
  - ✅ Joint content — [`docs/ecosystem-partnerships.md` § 4](./docs/ecosystem-partnerships.md#4-joint-content) lists co-authored pages, their locations, and the rules for joint content.
  - ✅ Cross-promotion — [`docs/ecosystem-partnerships.md` § 5](./docs/ecosystem-partnerships.md#5-cross-promotion) defines the reciprocal promotion model, discovery surfaces, and the opt-out path.
  - ✅ Ecosystem building — [`docs/ecosystem-partnerships.md` § 6](./docs/ecosystem-partnerships.md#6-ecosystem-building) maps partnership work onto the community programs that support it.
## Featured Projects

### Issue #406: Feature first projects

- **Priority:** Medium
- **Status:** ✅ Complete

- **Current state:** Deliverables created in-repo — [`docs/featured-projects.md`](./docs/featured-projects.md) features the first 5 projects with a case study, a runnable demo path and capture spec, a structured developer interview per project, and the social-promotion plan. Summarized in [`CONTRIBUTING.md`](./CONTRIBUTING.md#featured-projects) and linked from the [`README.md` Community Health & Metrics table](./README.md#-community-health--metrics). Tracked on GitHub as issue [#963](https://github.com/Soroban-Cookbook/Soroban-Cookbook-/issues/963).
- **Implementation hints:** Outcomes documented in README and `CONTRIBUTING.md`; external resources linked.
- **Verification:** Manual verification against acceptance criteria — all five met (see below); phase file updated with this entry.
- **Scope:** M
- **Description:** Feature initial projects
- **Acceptance Criteria:**
  - ✅ 3-5 projects featured — [`docs/featured-projects.md` § 2](./docs/featured-projects.md#2-the-first-five-featured-projects) features 5 projects with area, template, and the cookbook patterns each demonstrates.
  - ✅ Case studies written — [`docs/featured-projects.md` § 3](./docs/featured-projects.md#3-case-studies) contains 5 case studies (problem, cookbook material, change applied, outcome, reference implementation).
  - ✅ Screenshots/demos — [`docs/featured-projects.md` § 4](./docs/featured-projects.md#4-screenshots-and-demos) gives a runnable demo path per project, a five-step capture spec, the `docs/images/` asset rules, and links to the existing walkthrough series.
  - ✅ Developer interviews — [`docs/featured-projects.md` § 5](./docs/featured-projects.md#5-developer-interviews) defines the interview template and publishes one structured summary per featured project.
  - ✅ Social promotion — [`docs/featured-projects.md` § 6](./docs/featured-projects.md#6-social-promotion) defines channels, weekly/monthly cadence, the post template, and where outcomes are tracked.

## Summary

**Total Issues Created:** 70 (Issues #379-#534, #629)
**Completed:** 6
**In Progress:** 0  
**Planned:** 60

**Phase 8 Status:** 📋 **12% COMPLETE (Ongoing)**

Community categories:

- Community Calls: 5 issues
- Bug Bounty: 5 issues
- Recognition: 5 issues
- Stellar Partnership: 5 issues
- Wallet Integration: 5 issues
- Project Showcase: 5 issues
- Grants Program: 4 issues
- Community Channels: 5 issues
- Events & Workshops: 4 issues
- Metrics: 5 issues
- Feedback: 4 issues
- Partnerships: 4 issues
- Success Milestones: 5 issues

**Target: 100+ stars, 50+ contributors, 10+ projects**
