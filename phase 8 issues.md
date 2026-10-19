# Phase 8: Community & Ecosystem -

**Phase Status:** 📋 PLANNED (Ongoing)  
**Completion:** 8% (4/50)
**Last Audit Date:** October 19, 2026

Community and ecosystem initiatives — no in-repo deliverables yet.

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

## Metrics & Analytics

### Issue #927: Track Documentation Metrics

- **Priority:** Medium
- **Status:** ✅ Complete

- **Current state:** Deliverables created in-repo (October 2026).
- **Implementation hints:** Analytics instrumentation for mdBook and webapp; dashboard and workflow for automated collection; metric definitions updated.
- **Verification:** All acceptance criteria met — see deliverables below.
- **Scope:** M
- **Description:** Monitor documentation usage across mdBook and webapp surfaces.
- **Acceptance Criteria:**
  - ✅ Page views tracked — [`book/analytics.js`](./book/analytics.js) tracks page views on mdBook; [`webapp/src/lib/analytics.ts`](./webapp/src/lib/analytics.ts) tracks page views on the Next.js webapp. Both use privacy-respecting `sendBeacon` with no cookies or PII.
  - ✅ Search queries analyzed — mdBook script instruments the `#searchbar` input; webapp module tracks search queries via `trackSearch()`. The [`documentation-metrics.yml`](./.github/workflows/documentation-metrics.yml) workflow parses search events for unique terms, top searches, and zero-result queries.
  - ✅ Popular content identified — Dashboard includes top 10 pages (weekly) and top 20 pages (monthly) tables, populated from GitHub Traffic API and analytics event logs.
  - ✅ User paths tracked — Both analytics scripts record user paths in `sessionStorage` (per-tab, non-persistent). The workflow computes entry pages, exit pages, average path length, and top 3-page flows.
  - ✅ Dashboard — [`docs/documentation-metrics.md`](./docs/documentation-metrics.md) provides page view tables, search query analysis, popular content rankings, user path analysis, rolling weekly data, monthly trends, zero-result search terms, and mdBook vs webapp comparison.
  - ✅ Verification — Metric definitions added to [`docs/community-metrics.md`](./docs/community-metrics.md) § 3.6 (14 documentation usage metrics). Links added to [`README.md`](./README.md) and [`CONTRIBUTING.md`](./CONTRIBUTING.md).

## Summary

**Total Issues Created:** 70 (Issues #379-#534, #629, #927)
**Completed:** 4
**In Progress:** 0  
**Planned:** 63

**Phase 8 Status:** 📋 **8% COMPLETE (Ongoing)**

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
