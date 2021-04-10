# Introduction to GTK+

This is an introductory GTK+ programming tutorial. The tutorial is written in the C programming language. It has been created and tested on Linux. The GTK+ programming tutorial is suited for novice and intermediate programmers. This tutorials covers GTK+ 2.
GTK+

GTK+ is a library for creating graphical user interfaces. The library is created in C programming language. The GTK+ library is also called the GIMP toolkit. Originally, the library was created while developing the GIMP image manipulation program. Since then, the GTK+ became one of the most popular toolkits under Linux and BSD Unix. Today, most of the GUI software in the open source world is created in Qt or in GTK+. The GTK+ is an object-oriented application programming interface. The object-oriented system is created with the Glib Object system, which is a base for the GTK+ library. GObject also enables to create language bindings for various other programming languages. Language bindings exist for C++, Python, Perl, Java, C#, and other programming languages.

The GTK+ itself depends on the following libraries:

- Glib
- Pango
- ATK
- GDK
- GdkPixbuf
- Cairo

The Glib is a general purpose utility library. It provides various data types, string utilities, enables error reporting, message logging, working with threads, and other useful programming features. Pango is a library which enables internationalisation. ATK is the accessibility toolkit; it provides tools which help physically challenged people work with computers. GDK is a wrapper around the low-level drawing and windowing functions provided by the underlying graphics system. On Linux, GDK lies between the X Server and the GTK+ library. It handles basic rendering such as drawing primitives, raster graphics, cursors, fonts, as well as window events, and drag-and-drop functionality. The GdkPixbuf library is a toolkit for image loading and pixel buffer manipulation. Cairo is a library for creating 2D vector graphics. It has been included in GTK+ since version 2.8.

Gnome and XFce desktop environments have been created using the GTK+ library. SWT and wxWidgets are well known programming frameworks that use GTK+. Prominent software applications that use GTK+ include Firefox or Inkscape.

## Compiling GTK+ applications

To compile GTK+ applications, we have a handy tool called pkg-config. The pgk-config returns metadata about installed libraries. If we want to use a specific library, it will provide us necessary dependent libraries and include files that we need. The pkg-config program retrieves information about packages from special metadata files.

```
$ gcc -o simple simple.c `pkg-config --libs --cflags gtk+-2.0`
```

This line compiles a basic program. The source code consists of one file—simple.c.

```
$ pkg-config --cflags gtk+-2.0 | xargs -n3
-pthread -I/usr/include/gtk-2.0 -I/usr/lib/x86_64-linux-gnu/gtk-2.0/include
-I/usr/include/atk-1.0 -I/usr/include/cairo -I/usr/include/gdk-pixbuf-2.0
-I/usr/include/pango-1.0 -I/usr/include/gio-unix-2.0/ -I/usr/include/freetype2
-I/usr/include/glib-2.0 -I/usr/lib/x86_64-linux-gnu/glib-2.0/include -I/usr/include/pixman-1
-I/usr/include/libpng12 -I/usr/include/harfbuzz
```

The --cflags parameter prints pre-processor and compile flags required to compile GTK+ programs, including flags for all their dependencies.

```
$ pkg-config --libs gtk+-2.0 | xargs -n5
-lgtk-x11-2.0 -lgdk-x11-2.0 -latk-1.0 -lgio-2.0 -lpangoft2-1.0
-lpangocairo-1.0 -lgdk_pixbuf-2.0 -lcairo -lpango-1.0 -lfontconfig
-lgobject-2.0 -lglib-2.0 -lfreetype
```

The --libs parameter lists the necessary libraries.
Version

The following program prints the version of the GTK+ and the Glib libraries.
version.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {

    gtk_init(&argc, &argv);

    g_printf("GTK+ version: %d.%d.%d\n", gtk_major_version, 
        gtk_minor_version, gtk_micro_version);
    g_printf("Glib version: %d.%d.%d\n", glib_major_version,
        glib_minor_version, glib_micro_version);    
        
    return 0;
}
```

The program uses buit-in constants.

```
$ ./version 
GTK+ version: 2.24.23
Glib version: 2.40.2
```

This is the output of the version program.

## Sources

- gtk.org
- gtkforums.com
- Gtk+ 2 reference

This was an introduction to the GTK+ library. 