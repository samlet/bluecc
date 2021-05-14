@0xef5e8c49b058b72e;
# A happy, object-oriented interface!

interface Node {}

interface Directory extends(Node) {
  list @0 () -> (list: List(Entry));
  struct Entry {
    name @0 :Text;
    file @1 :Node;
  }

  create @1 (name :Text) -> (node :Node);
  open @2 (name :Text) -> (node :Node);
  delete @3 (name :Text);
  link @4 (name :Text, node :Node);
}

interface File extends(Node) {
  size @0 () -> (size: UInt64);
  read @1 (startAt :UInt64, amount :UInt64) -> (data: Data);
  write @2 (startAt :UInt64, data :Data);
  truncate @3 (size :UInt64);
}
