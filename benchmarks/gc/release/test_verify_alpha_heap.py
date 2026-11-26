import unittest

from verify_alpha_heap import check_tree


class CollectorIdentityTests(unittest.TestCase):
    workspace_gc = "ristretto_gc v0.35.0 (/workspace/ristretto_gc)"
    registry_gc = "ristretto_gc v0.35.0"
    other_checkout = "ristretto_gc v0.35.0 (/other/ristretto_gc)"

    def test_multiple_edges_to_the_workspace_collector(self):
        check_tree(f"{self.workspace_gc}|\n{self.workspace_gc}| (*)", self.workspace_gc)

    def test_graph_must_resolve_exactly_the_workspace_collector(self):
        for graph in ("ristretto_vm v0.35.0|", self.registry_gc + "|",
                      self.other_checkout + "|",
                      f"{self.workspace_gc}|\n{self.registry_gc}|",
                      f"{self.workspace_gc}|\nristretto_gc_catalog v0.35.0|",
                      f"{self.workspace_gc}|\nristretto_vm v0.35.0|experimental-canonical-heap"):
            with self.subTest(graph=graph), self.assertRaises(ValueError):
                check_tree(graph, self.workspace_gc)
