# GtkTextView widget

In this part of the GTK+ programming tutorial, we work with the GtkTextView widget.

GtkTextView widget is used for displaying and editing multiline text. GtkTextView widget has also the MVC design. The GtkTextView represents the view component and the GtkTextBuffer represents the model component. GtkTextBuffer is used to manipulate textual data. GtkTextTag is an attribute that can be applied to the text. GtkTextIter represents a position between two characters in the text. All manipulation with the text is done using text iterators.

## Simple example

In our first example, we show some of the GtkTextView's functionality. We show how to apply various text tags to the text data.
simpletextview.c

```cpp
#include <gtk/gtk.h>

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *view;
  GtkWidget *vbox;
  
  GtkTextBuffer *buffer;
  GtkTextIter start, end;
  GtkTextIter iter;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);
  gtk_window_set_title(GTK_WINDOW(window), "GtkTextView");

  vbox = gtk_vbox_new(FALSE, 0);
  view = gtk_text_view_new();
  gtk_box_pack_start(GTK_BOX(vbox), view, TRUE, TRUE, 0);

  buffer = gtk_text_view_get_buffer(GTK_TEXT_VIEW(view));

  gtk_text_buffer_create_tag(buffer, "gap",
        "pixels_above_lines", 30, NULL);

  gtk_text_buffer_create_tag(buffer, "lmarg", 
      "left_margin", 5, NULL);
  gtk_text_buffer_create_tag(buffer, "blue_fg", 
      "foreground", "blue", NULL); 
  gtk_text_buffer_create_tag(buffer, "gray_bg", 
      "background", "gray", NULL); 
  gtk_text_buffer_create_tag(buffer, "italic", 
      "style", PANGO_STYLE_ITALIC, NULL);
  gtk_text_buffer_create_tag(buffer, "bold", 
      "weight", PANGO_WEIGHT_BOLD, NULL);

  gtk_text_buffer_get_iter_at_offset(buffer, &iter, 0);

  gtk_text_buffer_insert(buffer, &iter, "Plain text\n", -1);
  gtk_text_buffer_insert_with_tags_by_name(buffer, &iter, 
        "Colored Text\n", -1, "blue_fg", "lmarg",  NULL);
  gtk_text_buffer_insert_with_tags_by_name (buffer, &iter, 
        "Text with colored background\n", -1, "lmarg", "gray_bg", NULL);

  gtk_text_buffer_insert_with_tags_by_name (buffer, &iter, 
        "Text in italics\n", -1, "italic", "lmarg",  NULL);

  gtk_text_buffer_insert_with_tags_by_name (buffer, &iter, 
        "Bold text\n", -1, "bold", "lmarg",  NULL);

  gtk_container_add(GTK_CONTAINER(window), vbox);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

The example shows some text with different GtkTextTags applied.

```cpp
view = gtk_text_view_new();
```

The gtk_text_view_new() function creates a new GtkTextView widget.

```cpp
gtk_text_buffer_create_tag(buffer, "blue_fg", 
    "foreground", "blue", NULL); 
```

The gtk_text_buffer_create_tag() function creates a tag and adds it to the tag table for the buffer. The second parameter is the tag name. The tag changes the colour of the text to blue.

```cpp
gtk_text_buffer_insert_with_tags_by_name(buffer, &iter, 
      "Colored Text\n", -1, "blue_fg", "lmarg",  NULL);
```

The gtk_text_buffer_insert_with_tags_by_name() function inserts text with blue_fg and lmarg text tags. The tags are recognized by their names.

[Figure: GtkTextView]()

## Lines and columns

The following example displays the current line and column of the text cursor.
linescols.c

```cpp
#include <gtk/gtk.h>

update_statusbar(GtkTextBuffer *buffer,
                  GtkStatusbar  *statusbar) {
  gchar *msg;
  gint row, col;
  GtkTextIter iter;
  
  gtk_statusbar_pop(statusbar, 0); 

  gtk_text_buffer_get_iter_at_mark(buffer,
      &iter, gtk_text_buffer_get_insert(buffer));

  row = gtk_text_iter_get_line(&iter);
  col = gtk_text_iter_get_line_offset(&iter);

  msg = g_strdup_printf("Col: %d Ln: %d", col+1, row+1);

  gtk_statusbar_push(statusbar, 0, msg);

  g_free(msg);
}

void mark_set_callback(GtkTextBuffer *buffer, 
    const GtkTextIter *new_location, GtkTextMark *mark, gpointer data) {
                       
  update_statusbar(buffer, GTK_STATUSBAR(data));
}

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *vbox;
  
  GtkWidget *toolbar;
  GtkWidget *view;
  GtkWidget *statusbar;
  GtkToolItem *exit;
  GtkTextBuffer *buffer;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 350, 300);
  gtk_window_set_title(GTK_WINDOW(window), "Lines & columns");

  vbox = gtk_vbox_new(FALSE, 0);
  gtk_container_add(GTK_CONTAINER(window), vbox);

  toolbar = gtk_toolbar_new();
  gtk_toolbar_set_style(GTK_TOOLBAR(toolbar), GTK_TOOLBAR_ICONS);

  exit = gtk_tool_button_new_from_stock(GTK_STOCK_QUIT);
  gtk_toolbar_insert(GTK_TOOLBAR(toolbar), exit, -1);

  gtk_box_pack_start(GTK_BOX(vbox), toolbar, FALSE, FALSE, 5);

  view = gtk_text_view_new();
  gtk_text_view_set_wrap_mode(GTK_TEXT_VIEW(view), GTK_WRAP_WORD);
  gtk_box_pack_start(GTK_BOX(vbox), view, TRUE, TRUE, 0);
  gtk_widget_grab_focus(view);

  buffer = gtk_text_view_get_buffer(GTK_TEXT_VIEW(view));

  statusbar = gtk_statusbar_new();
  gtk_box_pack_start(GTK_BOX(vbox), statusbar, FALSE, FALSE, 0);

  g_signal_connect(G_OBJECT(exit), "clicked", 
        G_CALLBACK(gtk_main_quit), NULL);

  g_signal_connect(buffer, "changed",
        G_CALLBACK(update_statusbar), statusbar);

  g_signal_connect_object(buffer, "mark_set", 
        G_CALLBACK(mark_set_callback), statusbar, 0);

  g_signal_connect_swapped(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  update_statusbar(buffer, GTK_STATUSBAR(statusbar));

  gtk_main();

  return 0;
}
```

In this code example, we show the current position of the text cursor in the statusbar.

```cpp
g_signal_connect(buffer, "changed",
      G_CALLBACK(update_statusbar), statusbar);
```

When we change the text, we call the update_statusbar() handler.

```cpp
g_signal_connect_object(buffer, "mark_set", 
      G_CALLBACK(mark_set_callback), statusbar, 0);
```

The mark_set signal is emitted when the cursor moves.

```cpp
gtk_statusbar_pop(statusbar, 0); 
```

This code line clears the message with context ID 0 from the statusbar.

```cpp
gtk_text_buffer_get_iter_at_mark(buffer,
    &iter, gtk_text_buffer_get_insert(buffer));

row = gtk_text_iter_get_line(&iter);
col = gtk_text_iter_get_line_offset(&iter);
```

These lines determine the current line and column.

```cpp
msg = g_strdup_printf("Col %d Ln %d", col+1, row+1);
```

The g_strdup_printf() is used to build the text to be displayed on the statusbar.

```cpp
gtk_statusbar_push(statusbar, 0, msg);
```

We show the text on the statusbar with the gtk_statusbar_push() function.

[Figure: Lines and columns]()

## Search & highlight

In the next example, we do some searching in the GtkTextBuffer; we highlight some text patterns in the text buffer.
search.c

```cpp
#include <gtk/gtk.h>
#include <gdk/gdkkeysyms.h>

gboolean key_pressed(GtkWidget *window,
    GdkEventKey* event, GtkTextBuffer *buffer) {

  GtkTextIter start_sel, end_sel;
  GtkTextIter start_find, end_find;
  GtkTextIter start_match, end_match;
  gboolean selected;    
  gchar *text;            
  
  if ((event->type == GDK_KEY_PRESS) && 
     (event->state & GDK_CONTROL_MASK)) {

    switch (event->keyval) {
        
      case GDK_m :
        selected = gtk_text_buffer_get_selection_bounds(buffer, 
            &start_sel, &end_sel);
      if (selected) {
        gtk_text_buffer_get_start_iter(buffer, &start_find);
        gtk_text_buffer_get_end_iter(buffer, &end_find);

        gtk_text_buffer_remove_tag_by_name(buffer, "gray_bg", 
            &start_find, &end_find);  
        text = (gchar *) gtk_text_buffer_get_text(buffer, &start_sel,
            &end_sel, FALSE);

        while (gtk_text_iter_forward_search(&start_find, text, 
                GTK_TEXT_SEARCH_TEXT_ONLY | 
                GTK_TEXT_SEARCH_VISIBLE_ONLY, 
                &start_match, &end_match, NULL)) {

          gtk_text_buffer_apply_tag_by_name(buffer, "gray_bg", 
              &start_match, &end_match);
          gint offset = gtk_text_iter_get_offset(&end_match);
          gtk_text_buffer_get_iter_at_offset(buffer, 
              &start_find, offset);
        }

        g_free(text);
      }

      break;

      case GDK_r:
        gtk_text_buffer_get_start_iter(buffer, &start_find);
        gtk_text_buffer_get_end_iter(buffer, &end_find);
      
        gtk_text_buffer_remove_tag_by_name(buffer, "gray_bg", 
            &start_find, &end_find);  
      break;
    }
  }

  return FALSE;
}

int main(int argc, gchar *argv[]) {

  GtkWidget *window;
  GtkWidget *view;
  GtkWidget *vbox;
  
  GtkTextBuffer *buffer;
  GtkTextIter start, end;
  GtkTextIter iter;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 350, 300);
  gtk_window_set_title(GTK_WINDOW(window), "Search & highlight");
  GTK_WINDOW(window)->allow_shrink = TRUE;

  vbox = gtk_vbox_new(FALSE, 0);
  view = gtk_text_view_new();
  gtk_widget_add_events(view, GDK_BUTTON_PRESS_MASK);
  gtk_box_pack_start(GTK_BOX(vbox), view, TRUE, TRUE, 0);

  buffer = gtk_text_view_get_buffer(GTK_TEXT_VIEW(view));
  gtk_text_buffer_create_tag(buffer, "gray_bg", 
      "background", "lightgray", NULL); 
  gtk_container_add(GTK_CONTAINER(window), vbox);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), NULL);

  g_signal_connect(G_OBJECT(window), "key-press-event",
        G_CALLBACK(key_pressed), buffer);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In our code example we use keyboard shortcuts. The Ctrl+M shortcut highlights the all occurrences of the currently selected text. The Ctrl+R removes the highlights from the text.

```cpp
gtk_text_buffer_create_tag(buffer, "gray_bg", 
    "background", "lightgray", NULL); 
```

This is the GtkTextTag that we use in our example. The tag makes the background of the text gray.

```cpp
selected = gtk_text_buffer_get_selection_bounds(buffer, 
    &start_sel, &end_sel);
```

With the gtk_text_buffer_get_selection_bounds() function, we get the start and end positions of the selected text.

```cpp
gtk_text_buffer_get_start_iter(buffer, &start_find);
gtk_text_buffer_get_end_iter(buffer, &end_find);
```

We get the first and the last position in the text buffer.

```cpp
gtk_text_buffer_remove_tag_by_name(buffer, "gray_bg", 
    &start_find, &end_find);  
```

With the gtk_text_buffer_remove_tag_by_name() funcion, we remove any previous text tag.

```cpp
text = (gchar *) gtk_text_buffer_get_text(buffer, &start_sel,
    &end_sel, FALSE);
```

We obtain the selected text. It is the text we are going to search for.

```cpp
while (gtk_text_iter_forward_search(&start_find, text, 
        GTK_TEXT_SEARCH_TEXT_ONLY | 
        GTK_TEXT_SEARCH_VISIBLE_ONLY, 
        &start_match, &end_match, NULL)) {

    gtk_text_buffer_apply_tag_by_name(buffer, "gray_bg", 
        &start_match, &end_match);
    gint offset = gtk_text_iter_get_offset(&end_match);
    gtk_text_buffer_get_iter_at_offset(buffer, 
        &start_find, offset);
}
```

This code searches for all occurences of the selected text. If we find any match, we apply the text tag. After the match, the ending point of the word becomes the starting point for the next search.

[Figure: Search & Highlight]()

In this chapter we covered the GtkTextView widget. 