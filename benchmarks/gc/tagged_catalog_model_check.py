#!/usr/bin/env python3
"""Reproduce current tagged catalog/pool models and executed fault controls.

Uses an isolated copy of the recorded Loom RMW correction. These models exclude
real pointer protection, concrete payloads, facade capacities and whole-GC progress.
No production dependency or collector source is changed by this runner.
"""
from pathlib import Path
import argparse
import json
import os
import re
import shutil
import subprocess

from catalog_model_check import run
from trace_certificate_check import ROOT, UPSTREAM, REVISION, digest

CATALOG_MODELS = 75
POOL_MODELS = 6
PATCHED_ATOMIC = '803c8b49841314e53ac110856f85a5eb298b542f0bb21717a7dfaf35c30a1813'

# name, package, target, [(relative file, exact old text, new text)], test, witness
FAULTS = [('fresh-cell-cas',
  'catalog',
  'lib',
  [('catalog/src/state.rs',
    'effect.cell.state.compare(&effect.expected, proposed)',
    'effect.cell.state.compare(&effect.cell.state.load().unwrap(), proposed)')],
  'tests::a_late_helper_cannot_replay_after_retirement_and_address_reuse',
  'assertion `left == right` failed'),
 ('late-certificate',
  'catalog',
  'lib',
  [('catalog/src/state.rs', '        completed.store(decision, Ordering::Release);', ''),
   ('catalog/src/state.rs',
    '        self.head.compare(busy, after);\n        after_release();',
    '        self.head.compare(busy, after);\n        after_release();\n        completed.store(decision, Ordering::Release);')],
  'tests::a_released_cancelled_successor_already_has_its_completion_certificate',
  'released cancellation successor exposed committed work'),
 ('fresh-slot-cas',
  'catalog',
  'lib',
  [('catalog/src/state.rs',
    'self.slots[slot].compare(Some(cell), None)',
    'self.slots[slot].compare(self.slots[slot].load().unwrap().as_ref(), None)')],
  'tests::cancelled_membership_cannot_remove_or_publish_a_reused_slot',
  'assertion failed: !catalog.release_retired(&old)'),
 ('pending-as-retry',
  'catalog',
  'model',
  [('catalog/src/state.rs', '_ => Outcome::Pending,', '_ => Outcome::Retry,')],
  'pending_request_recovers_after_effect_acquisition_failure',
  'left: Retry'),
 ('discovery-as-empty',
  'catalog',
  'model',
  [('catalog/src/state.rs',
    'self.slots.get(slot).map_or(Ok(None), OptionalLink::load)',
    'Ok(self.slots.get(slot).and_then(|entry| entry.load().ok().flatten()))')],
  'discovery_failure_reports_contention_without_hiding_membership',
  'Err(Contended)'),
 ('discard-prepared-outcomes',
  'catalog',
  'lib',
  [('catalog/src/state.rs', 'self.operation = Some(operation);', 'drop(operation);')],
  'tests::failed_final_record_allocation_retains_both_prepared_outcomes',
  'left: (30, 1)'),
 ('refresh-prepared-controller',
  'catalog',
  'lib',
  [('catalog/src/state.rs',
    'expected: self.expected.take().expect("unconsumed preparation"),',
    'expected: catalog.head.load().expect("quiescent fault"),')],
  'tests::a_partially_prepared_operation_cannot_refresh_its_expected_phase',
  'left: Committed'),
 ('discard-private-membership',
  'catalog',
  'lib',
  [('catalog/src/state.rs', 'self.private_cell = Some(failure.value);', 'drop(failure.value);')],
  'bounded::tests::private_cell_creation_retains_its_membership_across_a_full_cell_pool',
  'private membership was discarded'),
 ('quota-before-state',
  'catalog',
  'lib',
  [('catalog/src/bounded.rs',
    '    state: RequestState<R, C>,\n'
    '    next_object_slot: usize,\n'
    '    next_root_slot: usize,\n'
    '    lease: Lease<R>,',
    '    lease: Lease<R>,\n'
    '    state: RequestState<R, C>,\n'
    '    next_object_slot: usize,\n'
    '    next_root_slot: usize,')],
  'bounded::tests::quota_is_not_returned_until_owned_request_storage_is_destroyed',
  'left: (2, 0)'),
 ('merge-helper-quota',
  'catalog',
  'lib',
  [('catalog/src/bounded.rs',
    'Role::Helper => (&self.helpers, self.capacities.helpers),',
    'Role::Helper => (&self.mutators, self.capacities.mutators),')],
  'bounded::tests::helpers_have_independent_registration_and_private_drop_releases_all_backing',
  'Err` value: Limit'),
 ('foreign-record-alias',
  'catalog',
  'lib',
  [('catalog/src/bounded.rs',
    'if record.catalog != catalog.identity {',
    'if false && record.catalog != catalog.identity {')],
  'bounded::tests::keys_do_not_retain_storage_or_alias_foreign_or_reused_slots',
  'right: Err(Foreign)'),
 ('undersized-control-reserve',
  'catalog',
  'lib',
  [('catalog/src/bounded.rs', 'controls: rounded(reserve)?,', 'controls: rounded(8)?,')],
  'bounded::tests::checked_capacity_derivation_leaves_slack_and_rejects_overflow',
  'controls.is_power_of_two()'),
 ('discard-pending-facade',
  'catalog',
  'lib',
  [('catalog/src/bounded.rs',
    'Outcome::Pending => Progress::Pending,\n'
    '                    Outcome::Retry => {\n'
    '                        self.state = RequestState::Starting(*command);',
    'Outcome::Pending => { self.state = RequestState::Idle; Progress::Pending },\n'
    '                    Outcome::Retry => {\n'
    '                        self.state = RequestState::Starting(*command);')],
  'bounded::tests::a_pending_installed_facade_request_keeps_its_original_completion',
  'right: Err(Busy)'),
 ('reject-valid-failure',
  'catalog',
  'lib',
  [('catalog/src/bounded.rs',
    '        Err(RequestError::Contended)\n    }\n}',
    '        Err(RequestError::Conflict)\n    }\n}')],
  'bounded::tests::failed_membership_acquisition_does_not_reject_a_valid_command',
  'left: Rejected(Conflict)'),
 ('exhaustion-as-contention',
  'catalog',
  'lib',
  [('pool/src/state.rs',
    'return Err(AllocationError::GenerationExhausted);',
    'return Err(AllocationError::Unavailable);')],
  'bounded::tests::exhausted_backing_returns_a_terminal_error_instead_of_pending_forever',
  'exhausted numeric backing did not return a terminal error'),
 ('reject-installed-after-exhaustion',
  'catalog',
  'lib',
  [('catalog/src/bounded.rs',
    'RequestState::Attempt(command, attempt) => {\n'
    '                match catalog.raw.commit_in(attempt, context) {',
    'RequestState::Attempt(command, attempt) => {\n'
    '                if catalog.raw.generation_exhaustion_observed() { return '
    'Progress::Rejected(RequestError::GenerationExhausted); }\n'
    '                match catalog.raw.commit_in(attempt, context) {')],
  'bounded::tests::installed_final_generation_work_finishes_after_an_exhausted_probe',
  'left: Rejected(GenerationExhausted)'),
 ('ignore-trace-floor',
  'catalog',
  'lib',
  [('catalog/src/trace.rs',
    'requested.max(TraceRetention::at_floor(domain.floor).ok_or(TraceError::Conflict)?);',
    'requested;')],
  'tests::canonical_trace_floors_couple_retention_with_counted_work',
  'right: Some(Final)'),
 ('omit-trace-ticket',
  'catalog',
  'lib',
  [('catalog/src/trace.rs',
    '            phase.revision.checked_add(1).ok_or(TraceError::Exhausted)?\n'
    '        };\n'
    '        *mark = TraceMark {',
    '            0\n        };\n        *mark = TraceMark {')],
  'tests::canonical_trace_floors_couple_retention_with_counted_work',
  'retention published without traversal work'),
 ('double-count-stronger-trace',
  'catalog',
  'lib',
  [('catalog/src/trace.rs', 'if mark.ticket == 0 {', 'if true {')],
  'tests::stronger_trace_discovery_replaces_ticket_without_double_counting',
  'left: 2'),
 ('accept-old-trace-ticket',
  'catalog',
  'lib',
  [('catalog/src/trace.rs', '            || mark.ticket != ticket\n', '')],
  'tests::stronger_trace_discovery_replaces_ticket_without_double_counting',
  'stale trace ticket accepted'),
 ('pool-ignore-generation-limit',
  'pool',
  'model',
  [('pool/src/state.rs',
    'Self(AtomicU64::new((MAX_GENERATION - limit) << BITS))',
    'Self(AtomicU64::new(0))')],
  'competing_claims_cannot_cross_a_configured_final_generation',
  'left: 1'),
 ('pool-stale-generation',
  'pool',
  'model',
  [('pool/src/state.rs', '            && old >> BITS == generation\n', '')],
  'exact_generation_acquisition_excludes_reused_storage',
  'assertion failed: !state.acquire(generation)'),
 ('pool-early-reuse',
  'pool',
  'model',
  [('pool/src/state.rs',
    '            self.0.load(Ordering::Acquire);',
    '            self.0.load(Ordering::Acquire);\n'
    '            self.0.store((old >> BITS) << BITS, Ordering::Release);')],
  'busy_state_excludes_acquisition_and_reuse_through_destruction',
  'state.claim().is_none()'),
 ('pool-missing-final-acquire',
  'pool',
  'model',
  [('pool/src/state.rs', '            self.0.load(Ordering::Acquire);', '')],
  'final_release_acquires_prior_owner_writes',
  'left: 0'),
 ('pool-missing-initial-acquire',
  'pool',
  'model',
  [('pool/src/state.rs',
    '.compare_exchange(old, old + 1, Ordering::Acquire, Ordering::Relaxed)',
    '.compare_exchange(old, old + 1, Ordering::Relaxed, Ordering::Relaxed)')],
  'acquisition_observes_initialized_value',
  'left: 0'),
 ('rotating-cell-slot',
  'catalog',
  'model',
  [('catalog/src/state.rs',
    'cell.slot.store(self.slot, Ordering::Relaxed);',
    'cell.slot.store(0, Ordering::Relaxed);')],
  'rotating_cell_announcements_preserve_exact_identity_under_collision',
  'assertion `left != right` failed'),
 ('rotating-root-slot',
  'catalog',
  'model',
  [('catalog/src/catalog_roots.rs',
    'root.slot.store(*slot, Ordering::Relaxed);',
    'root.slot.store(0, Ordering::Relaxed);')],
  'rotating_root_announcements_preserve_target_and_location_under_collision',
  'assertion `left != right` failed'),
 ('rotating-reference-role',
  'catalog',
  'model',
  [('catalog/src/catalog_roots.rs',
    'if rotate && !matches!(self.location, RootLocation::External(_)) {',
    'if false && rotate && !matches!(self.location, RootLocation::External(_)) {')],
  'rotating_root_rejection_keeps_reserved_reference_role_usable',
  'CreationError::FixedLocation')]


FAULTS.extend([('batch-scan-bound',
  'catalog',
  'lib',
  [('catalog/src/catalog_roots.rs',
    'self.begin_scan_roots(snapshot, domain, 8)',
    'self.begin_scan_roots(snapshot, domain, 9)')],
  'root_tests::scan_batch_bounds_empty_ranges_and_preserves_single_slot_api',
  'left: 10'),
 ('batch-scan-lost-effect',
  'catalog',
  'lib',
  [('catalog/src/catalog_roots.rs',
    'if effect.is_some() || root_effect.is_some() || cursor == self.root_capacity() {',
    'if cursor == self.root_capacity() {')],
  'root_tests::scan_batch_stops_at_new_epoch_shading_effect_before_later_slots',
  'left: 8'),
 ('batch-scan-failed-discovery',
  'catalog',
  'model',
  [('catalog/src/catalog_roots.rs',
    'let root = self.discover_root(cursor).ok()?;',
    'let root = self.discover_root(cursor).ok().flatten();')],
  'scan_batch_failure_and_cancellation_cannot_skip_unobserved_slots',
  'assertion failed: catalog.begin_scan_root_batch(current, 0).is_none()')])


FAULTS.append(('batch-scan-lost-root-effect',
 'catalog',
 'lib',
 [('catalog/src/catalog_roots.rs',
   'if effect.is_some() || root_effect.is_some() || cursor == self.root_capacity() {',
   'if effect.is_some() || cursor == self.root_capacity() {')],
 'root_tests::scan_batch_stops_at_live_removal_without_revoking_later_root',
 'left: 8'))


FAULTS.extend([('edge-lost-effect',
  'catalog',
  'model',
  [('catalog/src/state.rs',
    'edge.apply(edge_proposed.expect("edge effect prepared before commit decision"));',
    'let _ = (edge, edge_proposed);')],
  'edge_effect_finishes_before_a_later_phase_can_be_observed',
  'phase escaped an incomplete edge effect'),
 ('edge-fresh-version-cas',
  'catalog',
  'lib',
  [('catalog/src/catalog_edges.rs',
    'self.slot.state.compare(&self.expected, proposed);',
    'self.slot.state.compare(&self.slot.state.load().unwrap(), proposed);')],
  'edge_tests::edge_stopped_effect_is_helped_and_cannot_overwrite_a_later_version',
  'late effect restored replaced edge'),
 ('edge-duplicate-same-target',
  'catalog',
  'lib',
  [('catalog/src/catalog_edges.rs',
    'if state.target != target\n            && let Some(key) = state.target',
    'if true\n            && let Some(key) = state.target')],
  'edge_tests::edge_same_target_installs_one_ticket_per_domain',
  'left: [2, 2]'),
 ('edge-orphan-failed-observation',
  'catalog',
  'lib',
  [('catalog/src/catalog_edges.rs',
    'self.edge_target(state.source)?',
    'self.edge_target(state.source).flatten()')],
  'edge_tests::edge_source_handoff_requires_publication_and_does_not_request_removal',
  'c.begin_orphan_edge_removal(s, edge).is_none()'),
 ('edge-scan-failed-observation',
  'catalog',
  'lib',
  [('catalog/src/catalog_edge_traversal.rs',
    'let Ok(edge) = catalog.discover_edge(self.first_slot + self.cursor) else {\n'
    '            return Ok(EdgeTraversalProgress::Pending);',
    'let Ok(edge) = catalog.discover_edge(self.first_slot + self.cursor) else {\n'
    '            self.cursor += 1;\n'
    '            return Ok(EdgeTraversalProgress::Pending);')],
  'edge_traversal_tests::edge_traversal_failed_observations_do_not_advance_or_acknowledge',
  'ignored observation failure 2'),
 ('edge-bounded-skip-traversal',
  'catalog',
  'lib',
  [('catalog/src/bounded.rs',
    '                } => {\n                    self.state = RequestState::Edge(edges::Request::Traverse {',
    '                } if false => {\n                    self.state = RequestState::Edge(edges::Request::Traverse {')],
  'bounded::edges::tests::bounded_acknowledgement_cannot_skip_a_transitive_edge',
  'bounded acknowledgement skipped graph traversal'),
 ('edge-handoff-keeps-removal',
  'catalog',
  'lib',
  [('catalog/src/catalog_edges.rs',
    'self.remove_on_drop = false;',
    'self.remove_on_drop = true;')],
  'edge_tests::edge_source_handoff_requires_publication_and_does_not_request_removal',
  '!observed.removal_requested()'),
 ('edge-scan-early-ack',
  'catalog',
  'lib',
  [('catalog/src/catalog_edge_traversal.rs',
    'if self.cursor == self.slots {',
    'if true {')],
  'bounded::edges::tests::bounded_acknowledgement_cannot_skip_a_transitive_edge',
  'bounded acknowledgement skipped graph traversal'),
 ('edge-orphan-live-source',
  'catalog',
  'model',
  [('catalog/src/catalog_edges.rs',
    '&& source.membership()? != Membership::Retired',
    '&& false && source.membership()? != Membership::Retired')],
  'edge_source_retirement_allows_orphan_cleanup_without_late_replay',
  'begin_orphan_edge_removal'),
 ('edge-child-wrong-target',
  'catalog',
  'lib',
  [('catalog/src/catalog_edges.rs',
    '|| state.target\n                != Some(ObjectKey {',
    '|| false && state.target\n                != Some(ObjectKey {')],
  'edge_tests::edge_child_root_rejects_replacement_and_source_revocation_races',
  'begin_root_edge_target')])


FAULTS.extend([
 ('graph-source-revision', 'catalog', 'lib',
  [('catalog/src/bounded/edges.rs',
    'if !owner.raw.validates_snapshot_in(&snapshot, context) {',
    'if false && !owner.raw.validates_snapshot_in(&snapshot, context) {')],
  'bounded::edges::tests::bounded_source_edge_observation_retries_replacement_and_revocation_windows',
  'source observation crossed a changed controller revision'),
 ('graph-discard-committed-request', 'catalog', 'lib',
  [('catalog/src/bounded.rs',
    'pub fn discard_request(&mut self) {\n        let _ = self.cancel();\n        self.state = RequestState::Idle;\n    }',
    'pub fn discard_request(&mut self) {\n        let _ = self.cancel();\n    }')],
  'bounded::edges::tests::bounded_discard_preserves_won_edge_commit_and_releases_participant_request',
  'discard retained a committed request'),
 ('graph-source-authority', 'catalog', 'lib',
  [('catalog/src/bounded/edges.rs', 'state.source == source_key', 'true')],
  'bounded::edges::tests::bounded_source_edge_observation_rejects_failed_or_foreign_authority',
  'right: Err(Conflict)'),
])



FAULTS.extend([
 ('indexed-publish-range', 'catalog', 'model',
  [('catalog/src/catalog_edges.rs',
    'if !self.edge_slot_matches_source(edge.slot, source.slot) {',
    'if false && !self.edge_slot_matches_source(edge.slot, source.slot) {')],
  'indexed_publication_cannot_escape_its_source_range',
  'indexed model accepted misplaced publication'),
 ('indexed-source-offset', 'catalog', 'lib',
  [('catalog/src/catalog_edge_traversal.rs', 'first_slot: range.start,', 'first_slot: 0,')],
  'edge_traversal_tests::indexed_traversal_checks_the_last_source_slot_without_scanning_other_sources',
  'last indexed edge was skipped'),
 ('indexed-short-range', 'catalog', 'lib',
  [('catalog/src/catalog_edge_traversal.rs', 'slots: range.len(),', 'slots: range.len().saturating_sub(1),')],
  'edge_traversal_tests::indexed_traversal_checks_the_last_source_slot_without_scanning_other_sources',
  'indexed traversal did not use the complete source range'),
 ('indexed-bounded-range', 'catalog', 'lib',
  [('catalog/src/bounded/edges.rs',
    'if !owner\n                .raw\n                .edge_slot_matches_source(edge.slot, source.target_slot())\n            {',
    'if false && !owner.raw.edge_slot_matches_source(edge.slot, source.target_slot()) {')],
  'bounded::edges::tests::bounded_indexed_publication_rejects_wrong_source_and_remains_usable',
  'bounded edge request did not settle'),
])

def valid_model_result(code, log, test=None, witness=None, expected=CATALOG_MODELS):
    """Accept one complete Rust test run, never diagnostics quoting its output."""
    starts = re.findall(r'^running ([0-9]+) tests?$', log, re.MULTILINE)
    results = re.findall(
        r'^test result: (ok|FAILED)\. ([0-9]+) passed; ([0-9]+) failed; '
        r'([0-9]+) ignored; ([0-9]+) measured; ([0-9]+) filtered out;[^\n]*$',
        log, re.MULTILINE)
    if len(starts) != 1 or len(results) != 1:
        return False
    status, passed, failed, ignored, measured, filtered = results[0]
    if ignored != '0' or measured != '0':
        return False
    if test is None:
        return (code == 0 and status == 'ok' and int(starts[0]) == expected
                and int(passed) == expected and failed == '0' and filtered == '0')
    return (code == 101 and status == 'FAILED' and starts == ['1']
            and passed == '0' and failed == '1'
            and re.search(r'^test ' + re.escape(test) + r' \.\.\. FAILED$', log, re.MULTILINE)
            is not None and bool(witness) and witness in log)


def source_files(directory):
    for current, folders, names in os.walk(directory):
        folders[:] = sorted(n for n in folders if n not in {'target', '.git', '__pycache__'})
        for name in sorted(names):
            path = Path(current) / name
            if path.is_file():
                yield path


def apply_changes(source, changes, write=False):
    """Validate every sequential anchor before modifying any file."""
    proposals = {}
    for name, old, new in changes:
        path = source / name
        if path not in proposals:
            proposals[path] = path.read_text()
        content = proposals[path]
        count = content.count(old)
        if not old or count != 1:
            raise ValueError(f'{name}: expected one mutation anchor, found {count}: {old!r}')
        proposals[path] = content.replace(old, new, 1)
    if write:
        for path, content in proposals.items():
            path.write_text(content)
    return {str(path.relative_to(source)): content for path, content in proposals.items()}


FAULTS += [
    ('helped-result-admission', 'catalog', 'lib',
     [('catalog/src/bounded/allocations.rs',
       'let _entered = match record.enter(context) {\n                        Ok(entered) => entered,\n                        Err(crate::RecordError::Contended) => return Step::Pending,\n                        Err(crate::RecordError::Foreign) => return fail(RequestError::Foreign),\n                        Err(crate::RecordError::Closed) => return fail(RequestError::Stale),\n                    };',
       'let _entered = record;')],
     'bounded::allocations::tests::helper_result_recovery_requires_admission_and_successful_observations',
     'contended admission returned a helped result'),
    ('helped-result-observation', 'catalog', 'lib',
     [('catalog/src/bounded/allocations.rs',
       'root.stably_live_in(context) != Some(true)',
       'root.stably_live_in(context).is_some_and(|_| false)')],
     'bounded::allocations::tests::helper_result_recovery_requires_admission_and_successful_observations',
     'failed stable observation returned a helped result'),
    ('stable-root-foreign', 'catalog', 'lib',
     [('catalog/src/catalog_roots.rs',
       'if !Arc::ptr_eq(&root.owner, &self.owner) {\n            return None;\n        }\n        let head = self.stable_head_in(context)?;',
       'let head = self.stable_head_in(context)?;')],
     'root_tests::stable_root_observation_rejects_failed_loads_and_foreign_catalogs',
     'foreign root produced a stable certificate'),
    ('phase-extra-backing', 'catalog', 'lib',
     [('catalog/src/state.rs', '        if self.effect.is_none()\n',
       '        if false && self.effect.is_none()\n')],
     'phase_tests::phase_only_requests_use_one_slot_without_retaining_a_predecessor_chain',
     'phase request reserved extra controllers'),
    ('phase-early-snapshot', 'catalog', 'lib',
     [('catalog/src/state.rs',
       'operation.decision.load(Ordering::Acquire) != UNDECIDED',
       'true || operation.decision.load(Ordering::Acquire) != UNDECIDED')],
     'phase_tests::stopped_phase_installation_is_helped_and_its_exact_result_survives_later_updates',
     'undecided phase escaped as a snapshot'),
    ('phase-ignores-cancel', 'catalog', 'lib',
     [('catalog/src/state.rs',
       '            let _ = operation.decision.compare_exchange(\n                UNDECIDED,\n                COMMIT,\n                Ordering::AcqRel,\n                Ordering::Acquire,\n            );',
       '            operation.decision.store(COMMIT, Ordering::Release);')],
     'phase_tests::phase_cancellation_before_or_after_install_preserves_the_cancelled_outcome',
     'right: Cancelled'),
    ('phase-private-completion', 'catalog', 'lib',
     [('catalog/src/state.rs',
       'Control::Phase(operation) => operation.decision.load(Ordering::Acquire),',
       'Control::Phase(operation) => operation.decision.load(Ordering::Acquire).max(COMMIT),')],
     'phase_tests::phase_failed_owning_acquisitions_never_imply_installation_or_completion',
     'right: Pending'),
    ('phase-late-head', 'catalog', 'lib',
     [('catalog/src/state.rs',
       '            after_effect();\n            after_certificate();\n            after_release();\n            return;',
       '            after_effect();\n            after_certificate();\n            self.head.compare(&self.head.load().unwrap(), share(busy).unwrap());\n            after_release();\n            return;')],
     'phase_tests::stopped_phase_installation_is_helped_and_its_exact_result_survives_later_updates',
     'late phase helper rewound the head'),
    ('phase-bypasses-object-effect', 'catalog', 'lib',
     [('catalog/src/state.rs', '        if self.effect.is_none()\n',
       '        if true\n')],
     'tests::stopped_install_and_effect_are_finished_before_phase_closure',
     'not live'),
]


FAULTS += [('phase-command-yield-window', 'catalog', 'lib', [('catalog/src/bounded.rs', 'self.state = if attempt.is_phase_only() {\n                            RequestState::Yielding(*command, PhaseYield::new(attempt))', 'self.state = if false && attempt.is_phase_only() {\n                            RequestState::Yielding(*command, PhaseYield::new(attempt))')], 'bounded::tests::phase_only_facade_requests_yield_twice_without_allocating_or_publishing', 'phase request did not yield before installation'), ('phase-root-yield-window', 'catalog', 'lib', [('catalog/src/bounded.rs', 'self.state = if attempt.is_phase_only() {\n                            RequestState::YieldingRoot(*command, PhaseYield::new(attempt))', 'self.state = if false && attempt.is_phase_only() {\n                            RequestState::YieldingRoot(*command, PhaseYield::new(attempt))')], 'bounded::tests::phase_only_facade_requests_yield_twice_without_allocating_or_publishing', 'phase request did not yield before installation')]

def standalone_manifest(path, checker):
    content = path.read_text()
    if not re.search(r'^\[workspace\]$', content, re.MULTILINE):
        content = '[workspace]\n\n' + content
    if '[patch.crates-io]' in content:
        raise ValueError(f'{path}: unexpected existing patch table')
    path.write_text(content + '\n[patch.crates-io]\nloom = { path = '
                    + json.dumps(str(checker)) + ' }\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--loom-source', type=Path)
    parser.add_argument('--catalog-source', type=Path,
                        default=ROOT / 'ristretto_gc/validation/catalog/catalog-tagged-prototype')
    parser.add_argument('--pool-source', type=Path,
                        default=ROOT / 'ristretto_gc/validation/catalog/catalog-tagged-pool-prototype')
    parser.add_argument('--timeout', type=int, default=900)
    parser.add_argument('--preflight-only', action='store_true',
                        help='check frozen mutation anchors without compiling or running tests')
    args = parser.parse_args()
    if args.timeout <= 0:
        parser.error('--timeout must be positive')
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    source = out / 'inputs'
    originals = {}
    for short, package in [('pool', args.pool_source.resolve()),
                           ('catalog', args.catalog_source.resolve())]:
        if not (package / 'Cargo.toml').is_file():
            parser.error(f'not a crate source: {package}')
        for file in source_files(package):
            originals[str(file)] = digest(file)
            copied = source / short / file.relative_to(package)
            copied.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(file, copied)
    manifest = source / 'catalog/Cargo.toml'
    dependency = 'path = "../catalog-tagged-pool-prototype"'
    if manifest.read_text().count(dependency) != 1:
        raise ValueError('catalog pool path no longer matches the declared source layout')
    manifest.write_text(manifest.read_text().replace(dependency, 'path = "../pool"'))
    inputs = {str(p.relative_to(source)): digest(p) for p in source_files(source)}
    failures = []
    for name, _, _, changes, _, _ in FAULTS:
        try:
            apply_changes(source, changes)
        except (ValueError, OSError) as error:
            failures.append({'name': name, 'error': str(error)})
    preflight = dict(originals=originals, inputs=inputs, fault_cases=len(FAULTS),
                     expected_catalog_models=CATALOG_MODELS, expected_pool_models=POOL_MODELS,
                     failures=failures, executed=False, qualification=False)
    (out / 'preflight.json').write_text(json.dumps(preflight, indent=2) + '\n')
    if failures:
        raise SystemExit('mutation preflight failed; see preflight.json')
    if args.preflight_only:
        print('Mutation anchors checked; no models or fault processes executed', flush=True)
        return
    if args.loom_source is None:
        registry = Path(os.environ.get('CARGO_HOME', str(Path.home() / '.cargo'))) / 'registry/src'
        matches = list(registry.glob('*/loom-0.7.2'))
        if len(matches) != 1:
            parser.error('pass --loom-source for installed Loom 0.7.2')
        args.loom_source = matches[0]
    checker = out / 'loom'
    shutil.copytree(args.loom_source, checker, ignore=shutil.ignore_patterns('target', '.git'))
    patch = out / 'loom-rmw-atomicity.patch'
    shutil.copy2(ROOT / 'benchmarks/gc/loom-rmw-atomicity.patch', patch)
    original = digest(checker / 'src/rt/atomic.rs')
    with (out / 'patch.log').open('wb') as log:
        subprocess.run(['patch', '-p1', '--batch', '--fuzz=0', '-i', str(patch)],
                       cwd=checker, stdout=log, stderr=subprocess.STDOUT, check=True)
    if digest(checker / 'src/rt/atomic.rs') != PATCHED_ATOMIC:
        raise SystemExit('patched checker atomic source does not match the recorded correction')
    checker_sources = {str(p.relative_to(checker)): digest(p) for p in source_files(checker)}
    sources, results, preparation, artifacts = {}, [], [], []
    # Run deliberate faults before spending time on exhaustive positive models.
    cases = [*FAULTS, ('pool-positive', 'pool', 'model', [], None, None),
             ('catalog-positive', 'catalog', 'model', [], None, None)]
    for name, package, target, changes, test, witness in cases:
        dst = out / name
        shutil.copytree(source, dst)
        apply_changes(dst, changes, write=True)
        for component in ['pool', 'catalog']:
            standalone_manifest(dst / component / 'Cargo.toml', checker)
        manifest = str(dst / package / 'Cargo.toml')
        explicit_environment = dict(CARGO_TARGET_DIR=str(out / (name + '-build')),
                                    CARGO_BUILD_JOBS='4', CARGO_INCREMENTAL='0')
        environment = dict(os.environ, **explicit_environment)
        resolved = run(['cargo', 'generate-lockfile', '--offline', '--manifest-path', manifest],
                       out / (name + '-resolve.log'), environment, 60)
        preparation.append(resolved)
        if resolved['exit_code'] != 0:
            (out / 'preparation.json').write_text(json.dumps(preparation, indent=2) + '\n')
            raise SystemExit(f'{name}: dependency resolution failed')
        sources[name] = {str(p.relative_to(dst)): digest(p) for p in source_files(dst)}
        command = ['cargo', 'test', '--offline', '--locked', '--manifest-path', manifest]
        command += ['--test', 'model'] if target == 'model' else ['--lib']
        if test:
            command += [test, '--', '--exact']
        print('start', name, flush=True)
        result = run(command, out / (name + '.log'), environment, args.timeout)
        log = Path(result['log']).read_text()
        count = POOL_MODELS if package == 'pool' else CATALOG_MODELS
        valid = valid_model_result(result['exit_code'], log, test, witness, count)
        result.update(name=name, valid=valid, negative=bool(test), expected_test=test,
                      expected_witness=witness, expected_count=None if test else count,
                      environment=explicit_environment)
        results.append(result)
        # Cargo prints the actual test executable; freeze it before later commands.
        executed = re.findall(r'^\s*Running .*?\(([^()]+)\)\s*$', log, re.MULTILINE)
        for index, path in enumerate(executed):
            binary = Path(path)
            if not binary.is_absolute():
                binary = Path.cwd() / binary
            if not binary.is_file():
                raise SystemExit(f'{name}: missing executed binary {binary}')
            saved = out / 'executed' / f'{name}-{index}.bin'
            saved.parent.mkdir(exist_ok=True)
            shutil.copy2(binary, saved)
            artifacts.append(dict(case=name, path=str(saved), source=str(binary),
                                  sha256=digest(saved), bytes=saved.stat().st_size))
        if valid and len(executed) != 1:
            raise SystemExit(f'{name}: expected one recorded test executable')
        unchanged = (all(digest(Path(p)) == sha for p, sha in originals.items())
                     and all(digest(source / p) == sha for p, sha in inputs.items())
                     and all(digest(out / variant / p) == sha
                             for variant, files in sources.items() for p, sha in files.items())
                     and all(digest(checker / p) == sha for p, sha in checker_sources.items()))
        record = dict(upstream=UPSTREAM, upstream_commit=REVISION, upstream_merged=False,
                      checker_status_scope='Recorded correction only; no current upstream status claim.',
                      original_atomic_sha256=original, patched_atomic_sha256=PATCHED_ATOMIC,
                      checker_sources=checker_sources, patch_sha256=digest(patch), originals=originals,
                      inputs=inputs, sources=sources, inputs_unchanged=unchanged,
                      preparation=preparation, runs=results, artifacts=artifacts,
                      qualification=False,
                      model_scope='Exact transition source; excludes actual pooled pointer protection, concrete payloads, facade capacities, callbacks and whole-GC progress.')
        (out / 'result.json').write_text(json.dumps(record, indent=2) + '\n')
        print('finish', name, result['exit_code'], valid, flush=True)
        if not valid or not unchanged:
            raise SystemExit(f'{name}: failed evidence check; see result.json')
    print(f'Completed {CATALOG_MODELS}+{POOL_MODELS} models and {len(FAULTS)} fault controls',
          flush=True)


if __name__ == '__main__':
    main()
