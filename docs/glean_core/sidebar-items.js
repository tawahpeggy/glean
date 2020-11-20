initSidebarItems({"enum":[["ErrorKind","A list enumerating the categories of errors in this crate."],["ErrorType","The possible error types for metric recording. Note: the cases in this enum must be kept in sync with the ones in the platform-specific code (e.g. ErrorType.kt) and with the metrics in the registry files."],["Lifetime","The supported metrics' lifetimes."]],"fn":[["global_glean","Gets a reference to the global Glean object."],["setup_glean","Sets or replaces the global Glean object."],["test_get_num_recorded_errors","Gets the number of recorded errors for the given metric and error type."]],"mod":[["metrics","The different metric types supported by the Glean SDK to handle data."],["ping","Ping collection, assembly & submission."],["storage","Storage snapshotting."],["traits","Important: consider this module unstable / experimental."],["upload","Manages the pending pings queue and directory."]],"struct":[["CommonMetricData","The common set of data shared across all different metric types."],["Configuration","The Glean configuration."],["Error","A specialized `Error` type for this crate's operations."],["Glean","The object holding meta information about a Glean instance."]],"type":[["Result","A specialized `Result` type for this crate's operations."]]});