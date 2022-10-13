#!/bin/perl

use strict;
use warnings;

local $/;
print "$_\n" for <STDIN> =~ /^.*((?:(?:Mon|Diens|Donners|Frei)tag|Mittwoch).*\.[\S\s]*?)(?=Dessert)/gm;
