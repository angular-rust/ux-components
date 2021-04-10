# GTK+ Widgets

In this part of the GTK+ programming tutorial, we will introduce some GTK+ widgets.

Widgets are basic building blocks of a GUI application. Over the years, several widgets became a standard in programming toolkits; for example a button, a check box, or a scroll bar. The GTK+ toolkit's philosophy is to keep the number of widgets at a minimum level. More specialised widgets are created as custom GTK+ widgets.

# GtkButton

GtkButton is a simple widget that is used to trigger an action.
button.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *halign;
  GtkWidget *btn;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_title(GTK_WINDOW(window), "GtkButton");
  gtk_window_set_default_size(GTK_WINDOW(window), 230, 150);
  gtk_container_set_border_width(GTK_CONTAINER(window), 15);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);

  halign = gtk_alignment_new(0, 0, 0, 0);
  gtk_container_add(GTK_CONTAINER(window), halign);

  btn = gtk_button_new_with_label("Quit");
  gtk_widget_set_size_request(btn, 70, 30);

  gtk_container_add(GTK_CONTAINER(halign), btn);

  g_signal_connect(G_OBJECT(btn), "clicked", 
      G_CALLBACK(gtk_main_quit), G_OBJECT(window));

  g_signal_connect(G_OBJECT(window), "destroy", 
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

The example shows a button that is positioned in the upper-left corner of the window. The application quits when we click on the button.

```cpp
btn = gtk_button_new_with_label("Quit");
```

The gtk_button_new_with_label() creates a new GtkButton with a label.

```cpp
g_signal_connect(G_OBJECT(btn), "clicked", 
    G_CALLBACK(gtk_main_quit), G_OBJECT(window));
```

The button's clicked signal is connected to the gtk_main_quit() function, which terminates the application.

[Figure: GtkButton]()

## GtkCheckButton

GtkCheckButton is a widget that has two states: on and off. The on state is visualized by a check mark.
checkbutton.c

```cpp
#include <gtk/gtk.h>

void toggle_title(GtkWidget *widget, gpointer window) {
    
  if (gtk_toggle_button_get_active(GTK_TOGGLE_BUTTON(widget))) {
    gtk_window_set_title(window, "GtkCheckButton");     
  } else {
    gtk_window_set_title(window, "");
  }
}

int main(int argc, char** argv) {

  GtkWidget *window;
  GtkWidget *halign;
  GtkWidget *cb;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_container_set_border_width(GTK_CONTAINER(window), 15);
  gtk_window_set_default_size(GTK_WINDOW(window), 230, 150);
  gtk_window_set_title(GTK_WINDOW(window), "GtkCheckButton");

  halign = gtk_alignment_new(0, 0, 0, 0);
  gtk_container_add(GTK_CONTAINER(window), halign);

  cb = gtk_check_button_new_with_label("Show title");
  gtk_toggle_button_set_active(GTK_TOGGLE_BUTTON(cb), TRUE);
  
  GTK_WIDGET_UNSET_FLAGS(cb, GTK_CAN_FOCUS);
  gtk_container_add(GTK_CONTAINER(halign), cb);

  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  g_signal_connect(cb, "clicked", 
      G_CALLBACK(toggle_title), (gpointer) window);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

The example displays a window title depending on the state of the GtkCheckButton.

```cpp
cb = gtk_check_button_new_with_label("Show title");
gtk_toggle_button_set_active(GTK_TOGGLE_BUTTON(cb), TRUE);
```

The GtkCheckButton is created and is marked by default; the title is initially shown.

```cpp
GTK_WIDGET_UNSET_FLAGS(cb, GTK_CAN_FOCUS);
```

This code line disables the focus.

```cpp
if (gtk_toggle_button_get_active(GTK_TOGGLE_BUTTON(widget))) {
  gtk_window_set_title(window, "GtkCheckButton");     
} else {
  gtk_window_set_title(window, "");
}
```

We show the title of the window, depending on the state of the GtkCheckButton. To set a title of the window, we use the gtk_window_set_title().

[Figure: GtkCheckButton]()

## GtkFrame

GtkFrame is a bin with a decorative frame and optional label.
frames.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *table;

  GtkWidget *frame1;
  GtkWidget *frame2;
  GtkWidget *frame3;
  GtkWidget *frame4;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 250, 250);
  gtk_window_set_title(GTK_WINDOW(window), "GtkFrame");
  
  gtk_container_set_border_width(GTK_CONTAINER(window), 10);

  table = gtk_table_new(2, 2, TRUE);
  gtk_table_set_row_spacings(GTK_TABLE(table), 10);
  gtk_table_set_col_spacings(GTK_TABLE(table), 10);
  gtk_container_add(GTK_CONTAINER(window), table);

  frame1 = gtk_frame_new("Shadow In");
  gtk_frame_set_shadow_type(GTK_FRAME(frame1), GTK_SHADOW_IN);
  frame2 = gtk_frame_new("Shadow Out");
  gtk_frame_set_shadow_type(GTK_FRAME(frame2), GTK_SHADOW_OUT);
  frame3 = gtk_frame_new("Shadow Etched In");
  gtk_frame_set_shadow_type(GTK_FRAME(frame3), GTK_SHADOW_ETCHED_IN);
  frame4 = gtk_frame_new("Shadow Etched Out");
  gtk_frame_set_shadow_type(GTK_FRAME(frame4), GTK_SHADOW_ETCHED_OUT);

  gtk_table_attach_defaults(GTK_TABLE(table), frame1, 0, 1, 0, 1);
  gtk_table_attach_defaults(GTK_TABLE(table), frame2, 0, 1, 1, 2);
  gtk_table_attach_defaults(GTK_TABLE(table), frame3, 1, 2, 0, 1);
  gtk_table_attach_defaults(GTK_TABLE(table), frame4, 1, 2, 1, 2);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), G_OBJECT(window));

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

The example shows four different frame types. The frames are attached into a table container.

```cpp
frame1 = gtk_frame_new("Shadow In");
```

The gtk_frame_new() function creates a GtkFrame with an optional label.

```cpp
gtk_frame_set_shadow_type(GTK_FRAME(frame1), GTK_SHADOW_IN);
```

The gtk_frame_set_shadow_type() function sets a shadow type for a frame.

[Figure: GtkFrame]()

## GtkHScale

GtkHScale is a horizontal slider widget for selecting a value from a range of values.
hscale.c

```cpp
void value_changed(GtkRange *range, gpointer win) {
    
   gdouble val = gtk_range_get_value(range);
   gchar *str = g_strdup_printf("%.f", val);    
   gtk_label_set_text(GTK_LABEL(win), str);
   
   g_free(str);
}

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *halign;
  GtkWidget *hbox;
  GtkWidget *hscale;
  GtkWidget *label;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 250);
  gtk_container_set_border_width(GTK_CONTAINER(window), 10);
  gtk_window_set_title(GTK_WINDOW(window), "GtkHScale");
  
  hbox = gtk_hbox_new(FALSE, 20);

  hscale = gtk_hscale_new_with_range(0, 100, 1);
  gtk_scale_set_draw_value(GTK_SCALE(hscale), FALSE);
  gtk_widget_set_size_request(hscale, 150, -1);
  label = gtk_label_new("...");
  gtk_misc_set_alignment(GTK_MISC(label), 0.0, 1);
  
  gtk_box_pack_start(GTK_BOX(hbox), hscale, FALSE, FALSE, 0);
  gtk_box_pack_start(GTK_BOX(hbox), label, FALSE, FALSE, 0);

  halign = gtk_alignment_new(0, 0, 0, 0);
  gtk_container_add(GTK_CONTAINER(halign), hbox);
  gtk_container_add(GTK_CONTAINER(window), halign);

  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);
      
  g_signal_connect(hscale, "value-changed",
      G_CALLBACK(value_changed), label);      

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In the example, we have a horizontal scale widget and a label widget. The currently chosen value is displayed in the label.

```cpp
gdouble val = gtk_range_get_value(range);
```

The gtk_range_get_value() function retrieves the currently selected value from the scale widget.

```cpp
gchar *str = g_strdup_printf("%.f", val);    
gtk_label_set_text(GTK_LABEL(win), str);
```

We build a string value with the g_strdup_printf() function and set it to the label with the gtk_label_set_text() function.

```cpp
hscale = gtk_hscale_new_with_range(0, 100, 1);
```

The gtk_hscale_new_with_range() function creates a new horizontal scale widget with the given range. The first parameter is the minimum value, the second parameter is the maximum value, and the last parameter is the step.

```cpp
gtk_scale_set_draw_value(GTK_SCALE(hscale), FALSE);
```

The gtk_scale_set_draw_value() specifies whether the current value is displayed as a string next to the slider. We turn the value off. Instead, we programmatically set it to the label widget.

[Figure: GtkHScale]()

## GtkLabel

The GtkLabel widget displays text.
label.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *label;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_title(GTK_WINDOW(window), "No sleep");
  gtk_container_set_border_width(GTK_CONTAINER(window), 15);

  label = gtk_label_new("I've always been too lame\n\
To see what's before me\n\
And I know nothing sweeter than\n\
Champaign from last New Years\n\
Sweet music in my ears\n\
And a night full of no fears\n\
\n\
But if I had one wish fulfilled tonight\n\
I'd ask for the sun to never rise\n\
If God passed a mic to me to speak\n\
I'd say \"Stay in bed, world,\n\
Sleep in peace");

  gtk_label_set_justify(GTK_LABEL(label), GTK_JUSTIFY_CENTER);
  gtk_container_add(GTK_CONTAINER(window), label);

  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```
The example shows two verses of a song.

```cpp
  label = gtk_label_new("I've always been too lame\n\
To see what's before me\n\
...
```

We create a GtkLabel widget. We can create multiline text label by using a new line character. Note the escape character. We use a rather long string and we do not want to put all the text into one line. In such cases, we can use an escape character.

```cpp
gtk_label_set_justify(GTK_LABEL(label), GTK_JUSTIFY_CENTER);
```

The gtk_label_set_justify() function aligns the text in the label. With the GTK_JUSTIFY_CENTER type, the text is centered.

[Figure: GtkLabel]()

## Label with markup

GtkLabel can also display markup language. The markup is the Pango text markup language.
markup.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *label;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 100);
  gtk_window_set_title(GTK_WINDOW(window), "Markup label");

  gchar *str = "<b>ZetCode</b>, knowledge only matters";

  label = gtk_label_new(NULL);
  gtk_label_set_markup(GTK_LABEL(label), str);

  gtk_container_add(GTK_CONTAINER(window), label);
  gtk_widget_show(label);

  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show(window);

  gtk_main();

  return 0;
}
```

The example shows a portion of text in bold.

```cpp
gchar *str = "<b>ZetCode</b>, knowledge only matters";
```

This is the string that is displayed. It contains a simple markup.

```cpp
label = gtk_label_new(NULL);
```

We create an empty label.

```cpp
gtk_label_set_markup(GTK_LABEL(label), str);
```

The gtk_label_set_markup() parses the string which is marked up and applies its attributes to the label.

[Figure: markup label]()

In this part of the GTK+ tutorial, we have covered GTK+ widgets. 