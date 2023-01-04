# grrs - a command line application

grrs is a simple command line application that searches a specified file for lines that contain a specified string.

## Usage

To use grrs, open a terminal window and navigate to the directory where grrs is located. Then, enter the following command:

```./grrs <string> <file path>```

Replace `<string>` with the string you want to search for, and `<file path>` with the path to the file you want to search.

## Example

Suppose you have a file called `my_file.txt` in your home directory, and you want to search for the string `hello`. You could do so by running the following command:

```./grrs hello ~my_file.txt```

This would search `my_file.txt` for any lines that contain the string `hello`, and print those lines to the terminal.

## Output

grrs will print each line that contains the search string on its own line in the terminal. If no lines are found that contain the search string, grrs will print a message indicating that no matches were found.

## Additional options

grrs does not have any additional options at this time. If you have any suggestions for additional features you would like to see, please feel free to open an issue on the grrs repository.
