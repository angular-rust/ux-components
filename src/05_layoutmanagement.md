# GTK+ layout management

In this chapter we show how to lay out our widgets in windows or dialogs.

When we design the UI of our application, we decide what widgets we will use and how we will organise those widgets. To organise our widgets, we use specialised non-visible widgets called layout containers. In this chapter, we will mention GtkAlignment, GtkFixed, GtkVBox, and GtkTable.

## GtkFixed

The GtkFixed container places child widgets at fixed positions and with fixed sizes. This container performs no automatic layout management. Therefore, it does not work with translations, font changes, or themes. In most applications, we do not use the GtkFixed container. There might be some specialised areas where the container can be used (for instance, positioning charts or images).
fixed.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *fixed;

  GtkWidget *btn1;
  GtkWidget *btn2;
  GtkWidget *btn3;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_title(GTK_WINDOW(window), "GtkFixed");
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);

  fixed = gtk_fixed_new();
  gtk_container_add(GTK_CONTAINER(window), fixed);

  btn1 = gtk_button_new_with_label("Button");
  gtk_fixed_put(GTK_FIXED(fixed), btn1, 150, 50);
  gtk_widget_set_size_request(btn1, 80, 30);

  btn2 = gtk_button_new_with_label("Button");
  gtk_fixed_put(GTK_FIXED(fixed), btn2, 15, 15);
  gtk_widget_set_size_request(btn2, 80, 30);

  btn3 = gtk_button_new_with_label("Button");
  gtk_fixed_put(GTK_FIXED(fixed), btn3, 100, 100);
  gtk_widget_set_size_request(btn3, 80, 30);

  g_signal_connect(G_OBJECT(window), "destroy", 
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In our example, we create three buttons and place them at fixed coordinates. When we resize the window of the application, the buttons keep their size and positions.

```cpp
fixed = gtk_fixed_new();
```

The get_fixed_new() function creates a GtkFixed container.

```cpp
gtk_fixed_put(GTK_FIXED(fixed), btn1, 150, 50);
```

The first button is placed using the gtk_fixed_put() function at coordinates x=150 and y=50.

```cpp
gtk_widget_set_size_request(btn1, 80, 30);
```

The gtk_widget_set_size_request() sets a minimum size for the widget. It is the smallest size a widget can accept while still functioning well and drawing itself correctly.

[Figure: GtkFixed container]()

## GtkAlignment

GtkAlignment controls the alignment of a widget. In addition, it can manage its scaling.
bottomleft.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *align;

  GtkWidget *lbl;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_title(GTK_WINDOW(window), "GtkAlignment");
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_container_set_border_width(GTK_CONTAINER(window), 5);

  align = gtk_alignment_new(0, 1, 0, 0);
  lbl = gtk_label_new("bottom-left");
  
  gtk_container_add(GTK_CONTAINER(align), lbl);
  gtk_container_add(GTK_CONTAINER(window), align);

  g_signal_connect(G_OBJECT(window), "destroy", 
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In the example, a label is positioned in the bottom-left corner of the window.

```cpp
align = gtk_alignment_new(0, 1, 0, 0);
```

The gtk_alignment_new() function creates the GtkAlignment container. The parameters take values from 0 to 1. The first parameter is the horizontal alignment, where 0 is left and 1 is right. The second parameter is the vertical alignment, where 0 is top and 1 is bottom. The third parameter is a horizontal scale, which is the amount that the child widget expands horizontally to fill up unused space. A value of 0 indicates that the child widget should never expand. The last parameter is the vertical scale.

```cpp
lbl = gtk_label_new("bottom-left");
```

A label widget is created with the gtk_label_new() function.

```cpp
gtk_container_add(GTK_CONTAINER(align), lbl);
```

The label is added to the GtkAlignment container.

```cpp
gtk_container_add(GTK_CONTAINER(window), align);
```

Finally, the alignment container is placed into the window.

[Figure: GtkAlignment]()

## GtkVBox

GtkVBox is a vertical box container. It places its child widgets into a single column. GtkHBox is a very similar container; it places its child widgets into a single row.
vbox.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *vbox;

  GtkWidget *settings;
  GtkWidget *accounts;
  GtkWidget *loans;
  GtkWidget *cash;
  GtkWidget *debts;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 230, 250);
  gtk_window_set_title(GTK_WINDOW(window), "GtkVBox");
  gtk_container_set_border_width(GTK_CONTAINER(window), 5);

  vbox = gtk_vbox_new(TRUE, 1);
  gtk_container_add(GTK_CONTAINER(window), vbox);

  settings = gtk_button_new_with_label("Settings");
  accounts = gtk_button_new_with_label("Accounts");
  loans = gtk_button_new_with_label("Loans");
  cash = gtk_button_new_with_label("Cash");
  debts = gtk_button_new_with_label("Debts");

  gtk_box_pack_start(GTK_BOX(vbox), settings, TRUE, TRUE, 0);
  gtk_box_pack_start(GTK_BOX(vbox), accounts, TRUE, TRUE, 0);
  gtk_box_pack_start(GTK_BOX(vbox), loans, TRUE, TRUE, 0);
  gtk_box_pack_start(GTK_BOX(vbox), cash, TRUE, TRUE, 0);
  gtk_box_pack_start(GTK_BOX(vbox), debts, TRUE, TRUE, 0);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), G_OBJECT(window));

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

This example packs five buttons into one column. If we resize the window of the application, the child widgets are resized as well.

```cpp
vbox = gtk_vbox_new(TRUE, 1);
```

The gtk_vbox_new() function creates a GtkVBox container. We set the homogeneous parameter to TRUE. This means that all our buttons will be of the same size. The spacing between widgets is set to 1 pixel.

```cpp
gtk_box_pack_start(GTK_BOX(vbox), settings, TRUE, TRUE, 0);
```

The gtk_box_pack_start() function adds a widget to the box. The first two parameters are the box container and the child widget. The next three parameters are expand, fill, and padding. Note that the fill parameter has no effect if the expand parameter is set to FALSE. Similarly, the expand parameter has no effect if we have created the container with homogeneous parameter set to TRUE. In our case, the Settings button is given extra space when the window is enlarged and the widget fills the additional area.

[Figure: GtkVBox container]()

## GtkTable

The GtkTable widget arranges widgets in rows and columns.
table.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *table;
  GtkWidget *button;

  gchar *values[16] = { "7", "8", "9", "/", 
     "4", "5", "6", "*",
     "1", "2", "3", "-",
     "0", ".", "=", "+"
  };

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 250, 180);
  gtk_window_set_title(GTK_WINDOW(window), "GtkTable");

  gtk_container_set_border_width(GTK_CONTAINER(window), 5);

  table = gtk_table_new(4, 4, TRUE);
  gtk_table_set_row_spacings(GTK_TABLE(table), 2);
  gtk_table_set_col_spacings(GTK_TABLE(table), 2);

  int i = 0;
  int j = 0;
  int pos = 0;

  for (i=0; i < 4; i++) {
    for (j=0; j < 4; j++) {
      button = gtk_button_new_with_label(values[pos]);
      gtk_table_attach_defaults(GTK_TABLE(table), button, j, j+1, i, i+1);
      pos++;
    }
  }

  gtk_container_add(GTK_CONTAINER(window), table);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In this example, we create a set of buttons that we see in calculators.

```cpp
table = gtk_table_new(4, 4, TRUE);
```

We create a new GtkTable widget with 4 rows and 4 columns. When we pass TRUE to the third parameter, all table cells are resized to the size of the cell containing the largest widget.

```cpp
gtk_table_set_row_spacings(GTK_TABLE(table), 2);
gtk_table_set_col_spacings(GTK_TABLE(table), 2);
```

We set some space between rows and columns.

```cpp
for (i=0; i < 4; i++) {
  for (j=0; j < 4; j++) {
    button = gtk_button_new_with_label(values[pos]);
    gtk_table_attach_defaults(GTK_TABLE(table), button, j, j+1, i, i+1 );
    pos++;
  }
}
```

This code creates 16 buttons and places them into the container. The gtk_table_attach_defaults() adds children to a table container with identical padding and expansion options.

[Figure: GtkTable]()

## Corner buttons

The next example places two buttons in the bottom-right corner of the window.
cornerbuttons.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *okBtn;
  GtkWidget *clsBtn;

  GtkWidget *vbox;
  GtkWidget *hbox;
  GtkWidget *halign;
  GtkWidget *valign;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 350, 200);
  gtk_window_set_title(GTK_WINDOW(window), "Corner buttons");
  gtk_container_set_border_width(GTK_CONTAINER(window), 10);

  vbox = gtk_vbox_new(FALSE, 5);

  valign = gtk_alignment_new(0, 1, 0, 0);
  gtk_container_add(GTK_CONTAINER(vbox), valign);
  gtk_container_add(GTK_CONTAINER(window), vbox);

  hbox = gtk_hbox_new(TRUE, 3);

  okBtn = gtk_button_new_with_label("OK");
  gtk_widget_set_size_request(okBtn, 70, 30);
  gtk_container_add(GTK_CONTAINER(hbox), okBtn);
  clsBtn = gtk_button_new_with_label("Close");
  gtk_container_add(GTK_CONTAINER(hbox), clsBtn);

  halign = gtk_alignment_new(1, 0, 0, 0);
  gtk_container_add(GTK_CONTAINER(halign), hbox);

  gtk_box_pack_start(GTK_BOX(vbox), halign, FALSE, FALSE, 0);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), G_OBJECT(window));

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}

In the example, we use one horizontal box, one vertical box, and two alignment containers.

```cpp
valign = gtk_alignment_new(0, 1, 0, 0);
```

This alignment container puts its child widget to the bottom.

```cpp
gtk_container_add(GTK_CONTAINER(vbox), valign);
```

Here we place the alignment widget into the vertical box.

```cpp
hbox = gtk_hbox_new(TRUE, 3);

okBtn = gtk_button_new_with_label("OK");
gtk_widget_set_size_request(okBtn, 70, 30);
gtk_container_add(GTK_CONTAINER(hbox), okBtn);
clsBtn = gtk_button_new_with_label("Close");
gtk_container_add(GTK_CONTAINER(hbox), clsBtn);
```

We create a horizontal box and put two buttons inside it. The gtk_widget_set_size_request() sets the minimum size of a widget. Since we have set the homogeneous parameter of a GtkHBox to TRUE, the other button is adjusted to the new size as well.

```cpp
halign = gtk_alignment_new(1, 0, 0, 0);
gtk_container_add(GTK_CONTAINER(halign), hbox);

gtk_box_pack_start(GTK_BOX(vbox), halign, FALSE, FALSE, 0);
```

This creates an alignment container that places its child widget to the right. We add the horizontal box into the alignment container and pack the alignment container into the vertical box. The alignment container can take only one child widget; therefore, we must also use boxes.

[Figure: Corner buttons]()

## Windows

Next we will create a more advanced example. We show a window that can be found in JDeveloper.

[Figure: Windows dialog in JDeveloper]()

The dialog shows all opened windows, or more precisely tabs in JDeveloper application.
windows.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *table;
  GtkWidget *title;
  GtkWidget *wins;
  
  GtkWidget *halign;
  GtkWidget *halign2;
  GtkWidget *valign;

  GtkWidget *actBtn;
  GtkWidget *clsBtn;
  GtkWidget *hlpBtn;
  GtkWidget *okBtn;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_widget_set_size_request (window, 300, 250);
  
  gtk_window_set_title(GTK_WINDOW(window), "Windows");

  gtk_container_set_border_width(GTK_CONTAINER(window), 15);

  table = gtk_table_new(6, 4, FALSE);
  gtk_table_set_col_spacings(GTK_TABLE(table), 3);
  gtk_table_set_row_spacing(GTK_TABLE(table), 0, 3);

  title = gtk_label_new("Windows");
  halign = gtk_alignment_new(0, 0, 0, 0);
  gtk_container_add(GTK_CONTAINER(halign), title);
  gtk_table_attach(GTK_TABLE(table), halign, 0, 1, 0, 1, 
      GTK_FILL, GTK_FILL, 0, 0);

  wins = gtk_text_view_new();
  gtk_text_view_set_editable(GTK_TEXT_VIEW(wins), FALSE);
  gtk_text_view_set_cursor_visible(GTK_TEXT_VIEW(wins), FALSE);
  gtk_table_attach(GTK_TABLE(table), wins, 0, 2, 1, 3, 
      GTK_FILL | GTK_EXPAND, GTK_FILL | GTK_EXPAND, 1, 1);

  actBtn = gtk_button_new_with_label("Activate");
  gtk_widget_set_size_request(actBtn, 50, 30);
  gtk_table_attach(GTK_TABLE(table), actBtn, 3, 4, 1, 2, 
      GTK_FILL, GTK_SHRINK, 1, 1);

  valign = gtk_alignment_new(0, 0, 0, 0);
  clsBtn = gtk_button_new_with_label("Close");
 
  gtk_widget_set_size_request(clsBtn, 70, 30);
  gtk_container_add(GTK_CONTAINER(valign), clsBtn);
  gtk_table_set_row_spacing(GTK_TABLE(table), 1, 3);
  gtk_table_attach(GTK_TABLE(table), valign, 3, 4, 2, 3, 
      GTK_FILL, GTK_FILL | GTK_EXPAND, 1, 1);

  halign2 = gtk_alignment_new(0, 1, 0, 0);
  hlpBtn = gtk_button_new_with_label("Help");
  gtk_container_add(GTK_CONTAINER(halign2), hlpBtn);
  gtk_widget_set_size_request(hlpBtn, 70, 30);
  gtk_table_set_row_spacing(GTK_TABLE(table), 3, 5);
  gtk_table_attach(GTK_TABLE(table), halign2, 0, 1, 4, 5, 
      GTK_FILL, GTK_FILL, 0, 0);

  okBtn = gtk_button_new_with_label("OK");
  gtk_widget_set_size_request(okBtn, 70, 30);
  gtk_table_attach(GTK_TABLE(table), okBtn, 3, 4, 4, 5, 
      GTK_FILL, GTK_FILL, 0, 0);

  gtk_container_add(GTK_CONTAINER(window), table);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), G_OBJECT(window));

  gtk_widget_show_all(window);
  gtk_main();

  return 0;
}
```

The example uses a table container and three alignment containers.

```cpp
table = gtk_table_new(6, 4, FALSE);
```

A GtkTable container is created. It has six rows and four columns.

```cpp
gtk_table_set_col_spacings(GTK_TABLE(table), 3);
```

The gtk_table_set_col_spacings() sets the space between every column in the table to 3.

```cpp
gtk_table_set_row_spacing(GTK_TABLE(table), 0, 3);
```

The gtk_table_row_spacing() sets space between the first and the second row.

```cpp
title = gtk_label_new("Windows");
halign = gtk_alignment_new(0, 0, 0, 0);
gtk_container_add(GTK_CONTAINER(halign), title);
gtk_table_attach(GTK_TABLE(table), halign, 0, 1, 0, 1, 
    GTK_FILL, GTK_FILL, 0, 0);
```

This code creates a left-aligned label. The label is placed in the first row and the first column of the GtkTable container.

```cpp
wins = gtk_text_view_new();
gtk_text_view_set_editable(GTK_TEXT_VIEW(wins), FALSE);
gtk_text_view_set_cursor_visible(GTK_TEXT_VIEW(wins), FALSE);
gtk_table_attach(GTK_TABLE(table), wins, 0, 2, 1, 3, 
    GTK_FILL | GTK_EXPAND, GTK_FILL | GTK_EXPAND, 1, 1);
```

The GtkText widget spans two rows and two columns. We make the widget non-editable using the gtk_text_view_set_editable() method and hide its cursor using the gtk_text_view_set_cursor_visible() method.

```cpp
valign = gtk_alignment_new(0, 0, 0, 0);
clsBtn = gtk_button_new_with_label("Close");

gtk_widget_set_size_request(clsBtn, 70, 30);
gtk_container_add(GTK_CONTAINER(valign), clsBtn);
gtk_table_set_row_spacing(GTK_TABLE(table), 1, 3);
gtk_table_attach(GTK_TABLE(table), valign, 3, 4, 2, 3, 
    GTK_FILL, GTK_FILL | GTK_EXPAND, 1, 1);
```

We put the Close button next to the text view widget into the fourth column. We add the button into the alignment widget so that we can align it to the top.

```cpp
halign2 = gtk_alignment_new(0, 1, 0, 0);
hlpBtn = gtk_button_new_with_label("Help");
gtk_container_add(GTK_CONTAINER(halign2), hlpBtn);
gtk_widget_set_size_request(hlpBtn, 70, 30);
gtk_table_set_row_spacing(GTK_TABLE(table), 3, 5);
gtk_table_attach(GTK_TABLE(table), halign2, 0, 1, 4, 5, 
    GTK_FILL, GTK_FILL, 0, 0);
```

The Help button is left-aligned. It is placed below the text widget. We put some space between the text widget and the button.

```cpp
okBtn = gtk_button_new_with_label("OK");
gtk_widget_set_size_request(okBtn, 70, 30);
gtk_table_attach(GTK_TABLE(table), okBtn, 3, 4, 4, 5, 
    GTK_FILL, GTK_FILL, 0, 0);
```

The OK button goes to the second column, below the Activate and Close buttons.

[Figure: Windows]()

This chapter was dedicated to layout management. 