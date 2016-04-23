#!/bin/sh

cargo build

function test_creates_a_new_alias_for_a_contact_with_an_existing_name() {
	cp testdata/aliases testdata/tmp
	# echo 'alias paris-numa NUMA Paris <communique@numa.co>' >> testdata/tmp
	# echo 'alias paris-numa NUMA Paris <communique@numa.co>' >> ./testaliases

	cat testdata/email | ./target/debug/alias-auto-add

	

	# rm testdata/tmp
}


test_creates_a_new_alias_for_a_contact_with_an_existing_name
