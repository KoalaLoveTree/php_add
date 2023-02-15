# PHP adding lib

Library that adds adding functions for php.

### How to install

Run following commands and module file will be generated and moved to php modules directory

```cargo build --release```

```phpize```

```make```

```make install```

After you need to open your ```php.ini``` file and add extension there:

```extension=/path/to/php/modules/php_add.so```

### Doc

Right now only available functions is:

```add(x,y)```

Where ```x``` and ```y``` are either ```double``` or ```int```.


## Examples

You can test module functionality simply by executing commands in your terminal:

```php -a```

```echo add_double(2.0, 2.0);```