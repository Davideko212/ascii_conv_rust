# ascii_conv_rust
Rust CLI App that converts images to ASCII art

## Arguments

### Required
- Input path
> `--input-path <PATH>` or `-i <PATH>`
- Output path
> `--output-path <PATH>` or `-o <PATH>`

### Optional
- Resolution
> `--resolution <WIDTHxHEIGHT>` or `-r <WIDTHxHEIGHT>`
- Big Charset
> `--big-charset` or `-b`
- Time
> `--time` or `-t`
- Preview
> `--preview` or `-p`

## Example Usage

### Example #1:

<table>
  <tr>
     <td>Input</td>
     <td>Output</td>
  </tr>
  <tr>
    <td><img src=".\examples\rustacean.png" width="300" height="200"></td>
    <td><img src=".\examples\ascii_rustacean.png" width="300" height="200"></td>
  </tr>
</table>

Command used:
`ascii_conv_rust -i .\rustacean.png -o .\ -r 110x73`

Because the argument `-b` wasn't given, a charset size of 10 was used. This causes minor differences in brightness to be invisible in the converted image.
<br>
This conversion took less than 1ms to complete during testing.

### Example #2:

<table>
  <tr>
     <td>Input</td>
     <td>Output</td>
  </tr>
  <tr>
    <td><img src=".\examples\tux.png" width="200" height="300"></td>
    <td><img src=".\examples\ascii_tux.png" width="200" height="300"></td>
  </tr>
</table>

Command used:
`ascii_conv_rust -i .\tux.png -o .\ -r 62x144 -b`

This time the argument `-b` was given, meaning that a charset size of 70 was used. This causes minor differences in brightness to be (atleast partially) visible in the converted image.
<br>
This conversion took 2ms to complete during testing.
