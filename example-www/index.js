import * as unixtime_wasm from "unixtime-wasm";

var get_unixtime_ns_now = unixtime_wasm.unixtime_ns_now;
var get_rfc3339_local = unixtime_wasm.get_rfc3339_local_from_unixtime_ns;
var unixtime_ns_now = get_unixtime_ns_now();
var parsed_rfc3339_local = get_rfc3339_local(unixtime_ns_now);

alert(
  "Current unixtime in ns is " +
    unixtime_ns_now +
    "\nParsed RFC3339 is " +
    parsed_rfc3339_local
);
