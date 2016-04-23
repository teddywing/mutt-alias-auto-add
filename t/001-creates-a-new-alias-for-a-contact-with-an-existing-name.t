#!/usr/bin/env perl

use strict;
use warnings;

use Test::More;
use File::Copy qw(copy);

use feature qw(say);

my $tmp_file = "./testdata/tmp";

# Setup: make a temporary alias file
copy("./testdata/aliases", $tmp_file);


# Append test alias to alises file
open(my $fh, '>>', $tmp_file) or die;
say $fh 'alias paris-numa NUMA Paris <communique@numa.co>';
close $fh;


my $output = `cat ./testdata/email | ./target/debug/alias-auto-add $tmp_file`;
ok !$?;

# Check that the program outputs the full email coming from STDIN
{
	open(my $fh, '<', './testdata/email') or die;
	local $/ = undef;
	my $email = <$fh>;

	is $output, $email;

	close $fh;
}

# Check that the aliases file includes an alias for the address from the input email
{
	open(my $fh, '<', $tmp_file) or die;

	my $last_line;
	while (my $line = readline $fh) {
		$last_line = $line;
	}

	is $last_line, 'alias paris-numa-2 NUMA Paris <communication@numa.co>' . "\n";

	close $fh;
}


# Teardown
unlink $tmp_file;


done_testing;
