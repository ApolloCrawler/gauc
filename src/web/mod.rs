extern crate clap;
extern crate iron;
extern crate router;

// GET  /bucket/<BUCKET_NAME>/doc - list
// GET  /bucket/<BUCKET_NAME>/doc/<ID> - get
// POST /bucket/<BUCKET_NAME>/doc/<ID> - upsert
// POST /bucket/<BUCKET_NAME>/doc/add/<ID> - add
// POST /bucket/<BUCKET_NAME>/doc/append/<ID> - append
// POST /bucket/<BUCKET_NAME>/doc/prepend/<ID> - append
// POST /bucket/<BUCKET_NAME>/doc/replace/<ID> - replace
// POST /bucket/<BUCKET_NAME>/doc/set/<ID> - set

pub fn start_web(args: &clap::ArgMatches) {
    let port: u16 = args.value_of("rest-port").unwrap().to_string().parse::<u16>().unwrap();
    println!("Starting REST Interface on port {}.", port);

    // let mut router = Router::new();
}
