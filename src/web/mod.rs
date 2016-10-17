extern crate clap;
extern crate iron;
extern crate router;

// Bucket REST Interface
//
// GET  /bucket/<BUCKET_NAME>/doc - list
// DELETE  /bucket/<BUCKET_NAME>/doc/<ID> - delete/remove
// GET  /bucket/<BUCKET_NAME>/doc/<ID> - get
// POST /bucket/<BUCKET_NAME>/doc/<ID> - upsert
// POST /bucket/<BUCKET_NAME>/doc/<ID>/add - add
// POST /bucket/<BUCKET_NAME>/doc/<ID>/append - append
// POST /bucket/<BUCKET_NAME>/doc/<ID>/prepend - append
// POST /bucket/<BUCKET_NAME>/doc/<ID>/replace - replace
// POST /bucket/<BUCKET_NAME>/doc/<ID>/set - set
// POST /bucket/<BUCKET_NAME>/doc/<ID>/upsert - upsert (explitcit)

pub fn start_web(args: &clap::ArgMatches) {
    let port: u16 = args.value_of("rest-port").unwrap().to_string().parse::<u16>().unwrap();
    println!("Starting REST Interface on port {}.", port);

    // let mut router = Router::new();
}
