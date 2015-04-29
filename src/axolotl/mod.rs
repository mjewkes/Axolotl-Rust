
struct AxolotlState<Key> {
	root : Key,

	Option<header_send> : Key,
	Option<header_receive> : Key,

	next_header_send : Key,
	next_header_receive : Key,

	chain_send : Key,
	chain_receive : Key,

	identity_send : Key,
	identity_receive : Key,

	ratchet_send : Key,
	ratchet_receive : Key,

	chain_number_send : u32,
	chain_number_receive : u32,

	previous_chain_length : u32,

	should_update_ratchet : bool,
}

struct PlainText(Vec<u8>);

struct AxolotlHeader<Key> {
	n : u32,
	pn : u32,
	ratchet : Key,
}

trait AxolotlKey {
	type CypherText;
	type CypherHeader;
	fn derive(&self) -> Self;
	fn mix(&self,other:&Self) -> Self;
	fn generate() -> Self;

	fn enc(&self, plaintext : &PlainText) -> CypherText;
	fn dec(&self, cyphertext : &CypherText) -> PlainText;

	fn enc(&self, header : &AxolotlHeader<Self>) -> CypherHeader;
	fn dec(&self, header : &CypherHeader) -> AxolotlHeader<Self>;
}