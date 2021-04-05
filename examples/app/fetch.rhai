let fetchoptions = fetch_options();
fetchoptions.url = "https://httpbin.org/get";
fetchoptions.headers = #{"Accept" : "application/json"};
let result = fetch(fetchoptions);
log(result.body.origin);
let retval = #{};
retval.url = fetchoptions.url;
retval