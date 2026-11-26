//! Reduced protocol models. The owning head CAS is abstracted by a Loom mutex.
//! This does not model ArcSwap internals, metadata leases, allocation or root graphs.
use loom::sync::{Arc,Mutex};
use loom::sync::atomic::{AtomicBool,Ordering};
use loom::thread;
#[path="../src/tree.rs"]
mod tree;
#[path="../src/state.rs"]
mod state;
use state::*;

struct Link(Mutex<Arc<Head>>);
impl Link {
    fn new(head: Arc<Head>) -> Self { Self(Mutex::new(head)) }
    fn load(&self) -> Arc<Head> { self.0.lock().unwrap().clone() }
    fn compare(&self, expected: &Arc<Head>, proposed: Arc<Head>) -> bool {
        let mut current=self.0.lock().unwrap();
        if Arc::ptr_eq(&current,expected) { *current=proposed;true } else { false }
    }
}

#[test]
fn a_helper_recovers_the_result_of_a_winner_stopped_after_the_cas() {
    loom::model(|| {
        let catalog=Arc::new(Catalog::new());
        let proposal=Arc::new(Catalog::insert(catalog.snapshot(),1,8).unwrap());
        let stalled={let catalog=catalog.clone();let proposal=proposal.clone();
            thread::spawn(move || catalog.commit_stopped_after_cas(&proposal))};
        let helped=catalog.commit(&proposal);
        stalled.join().unwrap();
        assert_eq!(helped,Outcome::Committed,"a failed CAS missed the stopped winner's result");
        assert_eq!(catalog.snapshot().entries.pending_count(0),1);
    });
}

#[test]
fn announcement_and_phase_advance_cannot_both_commit_against_the_old_head() {
    loom::model(|| {
        let catalog=Arc::new(Catalog::new());
        let announce=Catalog::insert(catalog.snapshot(),1,8).unwrap();
        let close=Catalog::advance(catalog.snapshot(),0).unwrap();
        let publisher={let catalog=catalog.clone();thread::spawn(move || catalog.commit(&announce))};
        let closed=catalog.commit(&close)==Outcome::Committed;
        let published=publisher.join().unwrap()==Outcome::Committed;
        assert_ne!(closed,published);
        let snapshot=catalog.snapshot();
        assert_eq!(snapshot.entries.pending_count(0),usize::from(published));
        assert_eq!(snapshot.domains[0].floor,u8::from(closed));
    });
}

#[test]
fn old_work_acknowledgement_cannot_clear_retargeted_work() {
    loom::model(|| {
        let catalog=Arc::new(Catalog::new());
        assert_eq!(catalog.commit(&Catalog::insert(catalog.snapshot(),1,8).unwrap()),Outcome::Committed);
        let snapshot=catalog.snapshot();let ticket=snapshot.entries.get(1).unwrap().work[0];
        let ack=Catalog::acknowledge(snapshot,1,0,ticket).unwrap();
        let require=Catalog::require(catalog.snapshot(),1,0).unwrap();
        let helper={let catalog=catalog.clone();thread::spawn(move || catalog.commit(&ack))};
        let replaced=catalog.commit(&require)==Outcome::Committed;
        let acknowledged=helper.join().unwrap()==Outcome::Committed;
        assert_ne!(replaced,acknowledged);
        let snapshot=catalog.snapshot();
        assert_eq!(snapshot.entries.pending_count(0),usize::from(replaced));
        if replaced { assert_ne!(snapshot.entries.get(1).unwrap().work[0],ticket); }
        assert_eq!(snapshot.entries.pending_count(1),1);
    });
}
