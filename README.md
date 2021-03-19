# bluecc

## quick start
```bash
$ cargo run -p entity-seed --bin seed
   Compiling entity-seed v0.1.0 (/Users/xiaofeiwu/sagas/projs/bluecc/entity-seed)
    Finished dev [unoptimized + debuginfo] target(s) in 7.65s
     Running `target/debug/seed`
entity-seed 0.1.0

USAGE:
    seed <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    all                Generate specific modules: sql-files, model, dto
    data-files         Build all seed data files into a compressed resource file
    entity             Show entity meta-info
    gen                Generate specific entity with the type name
    help               Prints this message or the help of the given subcommand(s)
    list               List all entities in a module
    list-data-files    List all data files and it's entities
    list-services      List all service names
    model-files        Build all entity schema into a compressed resource file
    seed               Show entity seed data
    service            Show service meta-info
    service-files      Build all service schema into a compressed resource file
    wrapper            Generate model-types

$ cargo run -p entity-seed --bin seed seed Person
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/seed seed Person`
.. execute => Seed { name: "Person" }
[2021-03-19T08:04:26Z INFO  entity_seed::meta::data_files_procs] size 1639132
doc OfbizSetupShippingData.xml has 1 Person
Person (1280..1304)
	partyId = _NA_
doc ManufacturingTestsData.xml has 2 Person
Person (2478..2555)
	partyId = TestManufAdmin
	firstName = Manufacturing
	lastName = Admin
Person (3459..3530)
	partyId = TestSupplyAdmin
	firstName = Supply
	lastName = Admin
doc OrderTestData.xml has 1 Person
Person (3122..3195)
	partyId = TestDemoCustomer
	firstName = Test
	lastName = Customer
doc PartyTestsData.xml has 2 Person

```

