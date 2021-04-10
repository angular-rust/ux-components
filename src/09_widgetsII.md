# GTK+ Widgets II


In this part of the GTK+ programming tutorial, we continue covering various GTK+ widgets.

## GktComboBoxText

GktComboBoxText is a widget that allows the user to choose from a list of options. The options are strings.
combobox.c

```cpp
#include <gtk/gtk.h>

void combo_selected(GtkWidget *widget, gpointer window) {
     
  gchar *text = gtk_combo_box_get_active_text(GTK_COMBO_BOX(widget));
  gtk_label_set_text(GTK_LABEL(window), text);
  g_free(text);
}

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *hbox;
  GtkWidget *vbox;
  GtkWidget *combo;
  GtkWidget *label;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_title(GTK_WINDOW(window), "GtkComboBox");
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_container_set_border_width(GTK_CONTAINER(window), 15);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);
  
  hbox = gtk_hbox_new(FALSE, 0);
  vbox = gtk_vbox_new(FALSE, 15);

  combo = gtk_combo_box_new_text();
  gtk_combo_box_append_text(GTK_COMBO_BOX(combo), "Ubuntu");
  gtk_combo_box_append_text(GTK_COMBO_BOX(combo), "Arch");
  gtk_combo_box_append_text(GTK_COMBO_BOX(combo), "Fedora");
  gtk_combo_box_append_text(GTK_COMBO_BOX(combo), "Mint");
  gtk_combo_box_append_text(GTK_COMBO_BOX(combo), "Gentoo");
  gtk_combo_box_append_text(GTK_COMBO_BOX(combo), "Debian");

  gtk_box_pack_start(GTK_BOX(vbox), combo, FALSE, FALSE, 0);

  label = gtk_label_new("...");
  gtk_box_pack_start(GTK_BOX(vbox), label, FALSE, FALSE, 0);
  
  gtk_box_pack_start(GTK_BOX(hbox), vbox, FALSE, FALSE, 0);
  gtk_container_add(GTK_CONTAINER(window), hbox);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), NULL);

  g_signal_connect(G_OBJECT(combo), "changed", 
        G_CALLBACK(combo_selected), (gpointer) label);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

The example shows a combo box and a label. The combo box has a list of six options. These are the names of Linux distros. The label widget shows the selected option from the combo box.

```cpp
combo = gtk_combo_box_text_new();
```

The gtk_combo_box_text_new() function creates a simple text-only combo box.

```cpp
gtk_combo_box_append_text(GTK_COMBO_BOX(combo), "Ubuntu");
```

The gtk_combo_box_text_append_text() function appends a string to the list of strings stored in the combo box.

```cpp
label = gtk_label_new("-");
```

A new label widget is created.

```cpp
gchar *text =  gtk_combo_box_get_active_text(GTK_COMBO_BOX(widget));
gtk_label_set_text(GTK_LABEL(window), text);
g_free(text);
```

We get the selected text and set the label text to it. The gtk_combo_box_get_active_text() function returns the currently active string in the combo box. We set the string to the label with the gtk_label_set_text() function.

[Figure: GktComboBoxText]()

## GtkHSeparator

The GtkHSeparator is a horizontal separator. It is a kind of an ornament widget. There is also a sister GtkVSeparator widget.
separator.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *label1;
  GtkWidget *label2;
  GtkWidget *hseparator;
  GtkWidget *vbox;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_title(GTK_WINDOW(window), "GtkHSeparator");
  gtk_window_set_resizable(GTK_WINDOW(window), FALSE);

  gtk_container_set_border_width(GTK_CONTAINER(window), 10);

  label1 = gtk_label_new("Zinc is a moderately reactive, blue gray metal \
that tarnishes in moist air and burns in air with a bright bluish-green flame,\
giving off fumes of zinc oxide. It reacts with acids, alkalis and other non-metals.\
If not completely pure, zinc reacts with dilute acids to release hydrogen.");

  gtk_label_set_line_wrap(GTK_LABEL(label1), TRUE);

  label2 = gtk_label_new("Copper is an essential trace nutrient to all high \
plants and animals. In animals, including humans, it is found primarily in \
the bloodstream, as a co-factor in various enzymes, and in copper-based pigments. \
However, in sufficient amounts, copper can be poisonous and even fatal to organisms.");

  gtk_label_set_line_wrap(GTK_LABEL(label2), TRUE);

  vbox = gtk_vbox_new(FALSE, 10);
  gtk_container_add(GTK_CONTAINER(window), vbox);

  hseparator = gtk_hseparator_new();

  gtk_box_pack_start(GTK_BOX(vbox), label1, FALSE, TRUE, 0);
  gtk_box_pack_start(GTK_BOX(vbox), hseparator, FALSE, TRUE, 10);
  gtk_box_pack_start(GTK_BOX(vbox), label2, FALSE, TRUE, 0);

  g_signal_connect_swapped(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), G_OBJECT(window));

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

The code example shows definitions of two chemical elements; they are separated by a horizontal separator. This makes the example more visually appealing.

```cpp
   label1 = gtk_label_new("Zinc is a moderately reactive, blue gray metal \
that tarnishes in moist air and burns in air with a bright bluish-green flame,\
giving off fumes of zinc oxide. It reacts with acids, alkalis and other non-metals.\
If not completely pure, zinc reacts with dilute acids to release hydrogen.");
```

We create the first label, the definition of the Zinc element.

```cpp
gtk_label_set_line_wrap(GTK_LABEL(label2), TRUE);
```

The gtk_label_set_line_wrap() function break lines if text exceeds the widget's size.

```cpp
hseparator = gtk_hseparator_new();
```

The gtk_hseparator_new() creates a new GtkHSeparator.

```cpp
gtk_box_pack_start(GTK_BOX(vbox), label1, FALSE, TRUE, 0);
gtk_box_pack_start(GTK_BOX(vbox), hseparator, FALSE, TRUE, 10);
gtk_box_pack_start(GTK_BOX(vbox), label2, FALSE, TRUE, 0);
```

We place the separator between the labels.

[Figure: GtkHSeparator]()

## GtkEntry

GtkEntry is a single line text entry field. This widget is used to enter textual data.
entry.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *table;

  GtkWidget *label1;
  GtkWidget *label2;
  GtkWidget *label3;

  GtkWidget *entry1;
  GtkWidget *entry2;
  GtkWidget *entry3;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_title(GTK_WINDOW(window), "GtkEntry");
  gtk_container_set_border_width(GTK_CONTAINER(window), 10);

  table = gtk_table_new(3, 2, FALSE);
  gtk_container_add(GTK_CONTAINER(window), table);

  label1 = gtk_label_new("Name");
  label2 = gtk_label_new("Age");
  label3 = gtk_label_new("Occupation");

  gtk_table_attach(GTK_TABLE(table), label1, 0, 1, 0, 1, 
      GTK_FILL | GTK_SHRINK, GTK_FILL | GTK_SHRINK, 5, 5);
  gtk_table_attach(GTK_TABLE(table), label2, 0, 1, 1, 2, 
      GTK_FILL | GTK_SHRINK, GTK_FILL | GTK_SHRINK, 5, 5);
  gtk_table_attach(GTK_TABLE(table), label3, 0, 1, 2, 3, 
      GTK_FILL | GTK_SHRINK, GTK_FILL | GTK_SHRINK, 5, 5);

  entry1 = gtk_entry_new();
  entry2 = gtk_entry_new();
  entry3 = gtk_entry_new();

  gtk_table_attach(GTK_TABLE(table), entry1, 1, 2, 0, 1, 
      GTK_FILL | GTK_SHRINK, GTK_FILL | GTK_SHRINK, 5, 5);
  gtk_table_attach(GTK_TABLE(table), entry2, 1, 2, 1, 2, 
      GTK_FILL | GTK_SHRINK, GTK_FILL | GTK_SHRINK, 5, 5);
  gtk_table_attach(GTK_TABLE(table), entry3, 1, 2, 2, 3, 
      GTK_FILL | GTK_SHRINK, GTK_FILL | GTK_SHRINK, 5, 5);

  gtk_widget_show_all(window);

  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_main();

  return 0;
}
```

In our example we show three text entries and three labels.

```cpp
table = gtk_table_new(3, 2, FALSE);
gtk_container_add(GTK_CONTAINER(window), table);
```

To organise our widgets, we use the table container widget.

```cpp
entry1 = gtk_entry_new();
```

The gtk_entry_new() function creates a new GtkEntry.

```cpp
gtk_table_attach(GTK_TABLE(table), entry1, 1, 2, 0, 1, 
    GTK_FILL | GTK_SHRINK, GTK_FILL | GTK_SHRINK, 5, 5);
gtk_table_attach(GTK_TABLE(table), entry2, 1, 2, 1, 2, 
    GTK_FILL | GTK_SHRINK, GTK_FILL | GTK_SHRINK, 5, 5);
gtk_table_attach(GTK_TABLE(table), entry3, 1, 2, 2, 3, 
    GTK_FILL | GTK_SHRINK, GTK_FILL | GTK_SHRINK, 5, 5);
```
We attach the widgets to the table widget.

[Figure: GtkEntry]()

## GtkImage

GtkImage is a widget used to display an image.
image.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *image;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_title(GTK_WINDOW(window), "Red Rock");

  image = gtk_image_new_from_file("redrock.jpg");
 
  gtk_container_add(GTK_CONTAINER(window), image);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In our example we show an image of a castle.

```cpp
image = gtk_image_new_from_file("redrock.png");
```

The gtk_image_new_from_file() creates a new GtkImage from the specified filename . If the file is not found or cannot be loaded, the resulting GtkImage displays a "broken image" icon.

```cpp
gtk_container_add(GTK_CONTAINER(window), image);
```

The image is added to the window container.

## GtkStatusbar

GtkStatusbar displays status information. It is placed at the bottom of the application window.
statusbar.c

```cpp
#include <gtk/gtk.h>

void button_pressed(GtkWidget *widget, gpointer window) {
    
   gchar *str;
   str = g_strdup_printf("%s button clicked", 
         gtk_button_get_label(GTK_BUTTON(widget)));

   gtk_statusbar_push(GTK_STATUSBAR(window),
         gtk_statusbar_get_context_id(GTK_STATUSBAR(window), str), str);
   g_free(str);
}

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *hbox;
  GtkWidget *vbox;
  GtkWidget *halign;
  GtkWidget *balign;
  GtkWidget *button1;
  GtkWidget *button2;
  GtkWidget *statusbar;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);
  gtk_window_set_title(GTK_WINDOW(window), "GtkStatusbar");

  vbox = gtk_vbox_new(FALSE, 0);

  hbox = gtk_hbox_new(FALSE, 0);
  gtk_container_add(GTK_CONTAINER(window), vbox);

  halign = gtk_alignment_new(0, 0, 0, 0);
  gtk_container_add(GTK_CONTAINER(halign), hbox);
  gtk_box_pack_start(GTK_BOX(vbox), halign, TRUE, TRUE, 5);

  button1 = gtk_button_new_with_label("OK");
  gtk_widget_set_size_request(button1, 70, 30 );
  button2 = gtk_button_new_with_label("Apply");
  gtk_widget_set_size_request(button2, 70, 30 );

  gtk_box_pack_start(GTK_BOX(hbox), button1, FALSE, FALSE, 5);
  gtk_box_pack_start(GTK_BOX(hbox), button2, FALSE, FALSE, 0);

  balign = gtk_alignment_new(0, 1, 1, 0);
  statusbar = gtk_statusbar_new();
  gtk_container_add(GTK_CONTAINER(balign), statusbar);
  gtk_box_pack_start(GTK_BOX(vbox), balign, FALSE, FALSE, 0);

  g_signal_connect(G_OBJECT(button1), "clicked", 
           G_CALLBACK(button_pressed), G_OBJECT(statusbar));

  g_signal_connect(G_OBJECT(button2), "clicked", 
           G_CALLBACK(button_pressed), G_OBJECT(statusbar));

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In the code example, there are two buttons and a statusbar. If we click on the button, a message is displayed in the statusbar. It says which button was pressed.

```cpp
gchar *str;
str = g_strdup_printf("Button %s clicked", 
      gtk_button_get_label(GTK_BUTTON(widget)));
```

The message is built with the g_strdup_printf() function. We get the label of the button with the gtk_button_get_label() function.

```cpp
gtk_statusbar_push(GTK_STATUSBAR(window),
     gtk_statusbar_get_context_id(GTK_STATUSBAR(window), str), str);
```

We show the message in the statusbar. The gtk_statusbar_push() function pushes a new message onto a statusbar’s stack. The function requires a context id, which is returned by the gtk_statusbar_get_context_id() function.

```cpp
statusbar = gtk_statusbar_new();
```

The gtk_statusbar_new() function creates a new GtkStatusbar widget.

[Figure: GtkStatusbar]()

## GtkIconView

GtkIconView is a widget which displays a list of icons in a grid. It uses a GtkListStore to store its data.
iconview.c

```cpp
#include <gtk/gtk.h>
#include <assert.h>

enum {
    
  COL_DISPLAY_NAME,
  COL_PIXBUF,
  NUM_COLS
};

GtkTreeModel *init_model(void) {
    
  GtkListStore *list_store;
  GdkPixbuf *p1, *p2, *p3, *p4;
  GtkTreeIter iter;
  GError *err = NULL;

  p1 = gdk_pixbuf_new_from_file("ubuntu.png", &err); 
  p2 = gdk_pixbuf_new_from_file("gnumeric.png", &err);
  p3 = gdk_pixbuf_new_from_file("blender.png", &err);
  p4 = gdk_pixbuf_new_from_file("inkscape.png", &err);

  assert(err==NULL);    

  list_store = gtk_list_store_new(NUM_COLS, 
      G_TYPE_STRING, GDK_TYPE_PIXBUF);
      
  gtk_list_store_append(list_store, &iter);
  gtk_list_store_set(list_store, &iter, COL_DISPLAY_NAME, 
      "Ubuntu", COL_PIXBUF, p1, -1);
  gtk_list_store_append(list_store, &iter);
  gtk_list_store_set(list_store, &iter, COL_DISPLAY_NAME, 
      "Gnumeric", COL_PIXBUF, p2, -1);
  gtk_list_store_append(list_store, &iter);
  gtk_list_store_set(list_store, &iter, COL_DISPLAY_NAME, 
      "Blender", COL_PIXBUF, p3, -1);
  gtk_list_store_append(list_store, &iter);
  gtk_list_store_set(list_store, &iter, COL_DISPLAY_NAME, 
      "Inkscape", COL_PIXBUF, p4, -1);
  
  g_object_unref(p1);
  g_object_unref(p2);
  g_object_unref(p3);
  g_object_unref(p4);    

  return GTK_TREE_MODEL(list_store);
}

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *icon_view;
  GtkWidget *sw;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  gtk_window_set_title(GTK_WINDOW(window), "IconView");
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_container_set_border_width(GTK_CONTAINER(window), 10);
  gtk_window_set_default_size(GTK_WINDOW(window), 350, 300);

  sw = gtk_scrolled_window_new(NULL, NULL);
  gtk_container_add(GTK_CONTAINER(window), sw);
  gtk_scrolled_window_set_policy(GTK_SCROLLED_WINDOW(sw),
      GTK_POLICY_AUTOMATIC, GTK_POLICY_AUTOMATIC);
  gtk_scrolled_window_set_shadow_type(GTK_SCROLLED_WINDOW(sw),
      GTK_SHADOW_IN);

  icon_view = gtk_icon_view_new_with_model(init_model());
  gtk_container_add(GTK_CONTAINER(sw), icon_view);
        
  gtk_icon_view_set_text_column(GTK_ICON_VIEW(icon_view),
      COL_DISPLAY_NAME);
  gtk_icon_view_set_pixbuf_column(GTK_ICON_VIEW(icon_view), COL_PIXBUF);
  gtk_icon_view_set_selection_mode(GTK_ICON_VIEW(icon_view), 
      GTK_SELECTION_MULTIPLE);

  g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);
        
  gtk_main();
        
  return 0;
}
```

The example displays 4 icons. The icons represent four prominent open source projects.

```cpp
p1 = gdk_pixbuf_new_from_file("ubuntu.png", &err); 
p2 = gdk_pixbuf_new_from_file("gnumeric.png", &err);
p3 = gdk_pixbuf_new_from_file("blender.png", &err);
p4 = gdk_pixbuf_new_from_file("inkscape.png", &err);
```

We load four images from the disk using the gdk_pixbuf_new_from_file() function.

```cpp
list_store = gtk_list_store_new(NUM_COLS, 
    G_TYPE_STRING, GDK_TYPE_PIXBUF);
```

The gtk_list_store_new() function creates a GtkListStore, which is a list model for the GtkTreeView and GtkIconView widgets. We store textual and pixbuf data.

```cpp
gtk_list_store_append(list_store, &iter);
gtk_list_store_set(list_store, &iter, COL_DISPLAY_NAME, 
    "ubuntu", COL_PIXBUF, p1, -1);
```

This code adds a new row into the model.

```cpp
icon_view = gtk_icon_view_new_with_model(init_model());
```

The gtk_icon_view_new_with_model() creates a new GtkIconView widget with a GtkTreeModel.

```cpp
gtk_container_add(GTK_CONTAINER(sw), icon_view);
```

The GtkIconView is a container widget. We add it into the GtkScrolledWindow.

```cpp
gtk_icon_view_set_text_column(GTK_ICON_VIEW(icon_view),
    COL_DISPLAY_NAME);
```

The gtk_icon_view_set_text_column() function sets which column is a string column.

```cpp
gtk_icon_view_set_pixbuf_column(GTK_ICON_VIEW(icon_view),
    COL_PIXBUF);
```

The gtk_icon_view_set_pixbuf_column() function sets which is the column with pixbufs.

```cpp
gtk_icon_view_set_selection_mode(GTK_ICON_VIEW(icon_view), 
    GTK_SELECTION_MULTIPLE);
```

The gtk_icon_view_set_selection_mode() sets the selection mode of the GtkIconView. Choosing the GTK_SELECTION_MULTIPLE mode, it is possible to choose multiple icons.

[Figure: IconView]()

In this part of the GTK+ tutorial, we have continued covering GTK+ widgets. 