disclaimer: this sucks this sucks this sucks this sucks this sucks this sucks this sucks this sucks this sucks this sucks this sucks this sucks this sucks

maybe ill make a better version in future idk

# eco

eco is a cli tool for converting simple data between formats.

example: `eco decimal-ascii 97 98 99 100` > `abcd`

or, a shorter version: `eco d-a 97 98 99 100` > `abcd`

you can also convert data passed through stdin, by passing no other data as args.

this however will not care about which `from` format is specified and instead just read raw data.

## help message

usage: `eco <from>-<to> <any data to be converted>`

`from` and `to` may be any characters in order of any allowed format, so "binary", "b" and "bin" will all reference the same format.

formats

- binary
- hex
- utf8 (or text)
- decimal
- ascii
- raw
- base64
