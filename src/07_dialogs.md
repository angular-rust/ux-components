# GTK+ dialogs

In this part of the GTK+ programming tutorial, we work with dialogs.

Dialog windows or dialogs are an indispensable part of most modern GUI applications. A dialog is defined as a conversation between two or more persons. In a computer application a dialog is a window which is used to "talk" to the application. A dialog is used to input data, modify data, change the application settings etc.

## Message dialogs

Message dialogs are convenient dialogs that provide messages to the user of the application. The message consists of textual and image data.
messagedialogs.c

```cpp
#include <gtk/gtk.h>

void show_info(GtkWidget *widget, gpointer window) {
    
  GtkWidget *dialog;
  dialog = gtk_message_dialog_new(GTK_WINDOW(window),
            GTK_DIALOG_DESTROY_WITH_PARENT,
            GTK_MESSAGE_INFO,
            GTK_BUTTONS_OK,
            "Download Completed");
  gtk_window_set_title(GTK_WINDOW(dialog), "Information");
  gtk_dialog_run(GTK_DIALOG(dialog));
  gtk_widget_destroy(dialog);
}

void show_error(GtkWidget *widget, gpointer window) {
    
  GtkWidget *dialog;
  dialog = gtk_message_dialog_new(GTK_WINDOW(window),
            GTK_DIALOG_DESTROY_WITH_PARENT,
            GTK_MESSAGE_ERROR,
            GTK_BUTTONS_OK,
            "Error loading file");
  gtk_window_set_title(GTK_WINDOW(dialog), "Error");
  gtk_dialog_run(GTK_DIALOG(dialog));
  gtk_widget_destroy(dialog);
}

void show_question(GtkWidget *widget, gpointer window) {
    
  GtkWidget *dialog;
  dialog = gtk_message_dialog_new(GTK_WINDOW(window),
            GTK_DIALOG_DESTROY_WITH_PARENT,
            GTK_MESSAGE_QUESTION,
            GTK_BUTTONS_YES_NO,
            "Are you sure to quit?");
  gtk_window_set_title(GTK_WINDOW(dialog), "Question");
  gtk_dialog_run(GTK_DIALOG(dialog));
  gtk_widget_destroy(dialog);
}

void show_warning(GtkWidget *widget, gpointer window) {
    
  GtkWidget *dialog;
  dialog = gtk_message_dialog_new(GTK_WINDOW(window),
            GTK_DIALOG_DESTROY_WITH_PARENT,
            GTK_MESSAGE_WARNING,
            GTK_BUTTONS_OK,
            "Unallowed operation");
  gtk_window_set_title(GTK_WINDOW(dialog), "Warning");
  gtk_dialog_run(GTK_DIALOG(dialog));
  gtk_widget_destroy(dialog);
}

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *table;

  GtkWidget *info;
  GtkWidget *warn;
  GtkWidget *que;
  GtkWidget *err;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 220, 150);
  gtk_window_set_title(GTK_WINDOW(window), "Message dialogs");

  table = gtk_table_new(2, 2, TRUE);
  gtk_table_set_row_spacings(GTK_TABLE(table), 2);
  gtk_table_set_col_spacings(GTK_TABLE(table), 2);

  info = gtk_button_new_with_label("Info");
  warn = gtk_button_new_with_label("Warning");
  que = gtk_button_new_with_label("Question");
  err = gtk_button_new_with_label("Error");

  gtk_table_attach(GTK_TABLE(table), info, 0, 1, 0, 1, 
      GTK_FILL, GTK_FILL, 3, 3);
  gtk_table_attach(GTK_TABLE(table), warn, 1, 2, 0, 1, 
      GTK_FILL, GTK_FILL, 3, 3);
  gtk_table_attach(GTK_TABLE(table), que, 0, 1, 1, 2, 
      GTK_FILL, GTK_FILL, 3, 3);
  gtk_table_attach(GTK_TABLE(table), err, 1, 2, 1, 2, 
      GTK_FILL, GTK_FILL, 3, 3);
  
  gtk_container_add(GTK_CONTAINER(window), table);
  gtk_container_set_border_width(GTK_CONTAINER(window), 15);

  g_signal_connect(G_OBJECT(info), "clicked", 
        G_CALLBACK(show_info), (gpointer) window); 

  g_signal_connect(G_OBJECT(warn), "clicked", 
        G_CALLBACK(show_warning), (gpointer) window); 

  g_signal_connect(G_OBJECT(que), "clicked", 
        G_CALLBACK(show_question), (gpointer) window); 

  g_signal_connect(G_OBJECT(err), "clicked", 
        G_CALLBACK(show_error), (gpointer) window); 

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), G_OBJECT(window));

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In our example, we show four kinds of message dialogs: Information, Warning, Question, and Error message dialogs.

```cpp
void show_question(GtkWidget *widget, gpointer window) {
    
  GtkWidget *dialog;
  dialog = gtk_message_dialog_new(GTK_WINDOW(window),
            GTK_DIALOG_DESTROY_WITH_PARENT,
            GTK_MESSAGE_QUESTION,
            GTK_BUTTONS_YES_NO,
            "Are you sure to quit?");
  gtk_window_set_title(GTK_WINDOW(dialog), "Question");
  gtk_dialog_run(GTK_DIALOG(dialog));
  gtk_widget_destroy(dialog);
}
```

In the show_question() function, we pop up the message dialog. The message dialog is created using the gtk_message_dialog_new() call. The parameters of the function specify what kind of message dialog we create. The GTK_MESSAGE_QUESTION constant creates a question type dialog. The GTK_BUTTONS_YES_NO constant will add Yes and No buttons in the dialog. The last parameter is the text that we display in the dialog. The gtk_dialog_run() function shows the dialog and blocks the main loop until the dialog responds or is destroyed. The dialog must be destroyed with the gtk_widget_destroy() function.
Information message dialog Warning message dialog Question message dialog 

[Figure: Message dialogs]()

## GtkAboutDialog

GtkAboutDialog is a dialog whose purpose is to display information about the application. It can display the logo, the name of the application, the version, the copyright, the website, and the licence information. It is also possible to give credits to the authors, documenters, translators, and artists.
aboutdialog.c

```cpp
#include <gtk/gtk.h>

void show_about(GtkWidget *widget, gpointer data) {

  GdkPixbuf *pixbuf = gdk_pixbuf_new_from_file("battery.png", NULL);

  GtkWidget *dialog = gtk_about_dialog_new();
  gtk_about_dialog_set_name(GTK_ABOUT_DIALOG(dialog), "Battery");
  gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(dialog), "0.9"); 
  gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(dialog),"(c) Jan Bodnar");
  gtk_about_dialog_set_comments(GTK_ABOUT_DIALOG(dialog), 
     "Battery is a simple tool for battery checking.");
  gtk_about_dialog_set_website(GTK_ABOUT_DIALOG(dialog), 
     "http://www.batteryhq.net");
  gtk_about_dialog_set_logo(GTK_ABOUT_DIALOG(dialog), pixbuf);
  g_object_unref(pixbuf), pixbuf = NULL;
  gtk_dialog_run(GTK_DIALOG (dialog));
  gtk_widget_destroy(dialog);
}

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *about;
  GdkPixbuf *battery;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 220, 150);
  gtk_window_set_title(GTK_WINDOW(window), "Battery");

  gtk_container_set_border_width(GTK_CONTAINER(window), 15);
  gtk_widget_add_events(window, GDK_BUTTON_PRESS_MASK);

  g_signal_connect(G_OBJECT(window), "button-press-event", 
        G_CALLBACK(show_about), (gpointer) window); 

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), G_OBJECT(window));

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

The code example uses a GtkAboutDialog with some of its features. A click on the client area of the window pops up the dialog.

```cpp
GtkWidget *dialog = gtk_about_dialog_new();
```

The GtkAboutDialog is created with the gtk_about_dialog_new() function.

```cpp
gtk_about_dialog_set_name(GTK_ABOUT_DIALOG(dialog), "Battery");
gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(dialog), "0.9"); 
gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(dialog), 
    "(c) Jan Bodnar");
```

These function calls set the name, the version, and the copyright of the application.

```cpp
gtk_about_dialog_set_comments(GTK_ABOUT_DIALOG(dialog), 
    "Battery is a simple tool for battery checking.");
gtk_about_dialog_set_website(GTK_ABOUT_DIALOG(dialog), 
    "http://www.batteryhq.net");
```
These lines set descriptive comments and the website of the application.

```cpp
GdkPixbuf *pixbuf = gdk_pixbuf_new_from_file("battery.png", NULL);
...
gtk_about_dialog_set_logo(GTK_ABOUT_DIALOG(dialog), pixbuf);
g_object_unref(pixbuf), pixbuf = NULL;
```

This code creates a logo of the application.

[Figure: GtkAboutDialog]()

## GtkFontSelectionDialog

GtkFontSelectionDialog is a dialog for selecting fonts. It is typically used in applications that do some text editing or formatting.
fontdialog.c

```cpp
#include <gtk/gtk.h>

void select_font(GtkWidget *widget, gpointer label) {
    
  GtkResponseType result;

  GtkWidget *dialog = gtk_font_selection_dialog_new("Select Font");

  result = gtk_dialog_run(GTK_DIALOG(dialog));

  if (result == GTK_RESPONSE_OK || result == GTK_RESPONSE_APPLY) {

     PangoFontDescription *font_desc;
     gchar *fontname = gtk_font_selection_dialog_get_font_name(
                            GTK_FONT_SELECTION_DIALOG(dialog));

     font_desc = pango_font_description_from_string(fontname);

     gtk_widget_modify_font(GTK_WIDGET(label), font_desc);

     g_free(fontname);
   }

  gtk_widget_destroy(dialog);
}

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *label;
  GtkWidget *vbox;

  GtkWidget *toolbar;
  GtkToolItem *font;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 280, 200);
  gtk_window_set_title(GTK_WINDOW(window), "Font Selection Dialog");

  vbox = gtk_vbox_new(FALSE, 0);
  gtk_container_add(GTK_CONTAINER(window), vbox);

  toolbar = gtk_toolbar_new();
  gtk_toolbar_set_style(GTK_TOOLBAR(toolbar), GTK_TOOLBAR_ICONS);

  gtk_container_set_border_width(GTK_CONTAINER(toolbar), 2);

  font = gtk_tool_button_new_from_stock(GTK_STOCK_SELECT_FONT);
  gtk_toolbar_insert(GTK_TOOLBAR(toolbar), font, -1);

  gtk_box_pack_start(GTK_BOX(vbox), toolbar, FALSE, FALSE, 5);

  label = gtk_label_new("ZetCode");
  gtk_label_set_justify(GTK_LABEL(label), GTK_JUSTIFY_CENTER);
  gtk_box_pack_start(GTK_BOX(vbox), label, TRUE, FALSE, 5);

  g_signal_connect(G_OBJECT(font), "clicked", 
        G_CALLBACK(select_font), label);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In the code example, we put a simple label into the center of the window. We show a font selecting dialog by clicking on the toolbar button.

```cpp
GtkWidget *dialog = gtk_font_selection_dialog_new("Select Font");
result = gtk_dialog_run(GTK_DIALOG(dialog));
```

We create the GtkFontSelectionDialog with the gtk_font_selection_dialog_new() function and run it with the gtk_dialog_run() function.

```cpp
if (result == GTK_RESPONSE_OK || result == GTK_RESPONSE_APPLY) {

    PangoFontDescription *font_desc;
    gchar *fontname = gtk_font_selection_dialog_get_font_name(
                        GTK_FONT_SELECTION_DIALOG(dialog));

    font_desc = pango_font_description_from_string(fontname);

    gtk_widget_modify_font(GTK_WIDGET(label), font_desc);

    g_free(fontname);
}
```

If the user clicks on the OK or Apply button, we proceed. We get the selected font name using the gtk_font_selection_dialog_get_font_name() function. Then we change the label's font to the selected font name.

[Figure: GtkFontSelectionDialog]()

## GtkColorSelectionDialog

GtkColorSelectionDialog is a dialog for selecting a colour.
colordialog.c

```cpp
#include <gtk/gtk.h>

void select_font(GtkWidget *widget, gpointer label) {

  GtkResponseType result;
  GtkColorSelection *colorsel;

  GtkWidget *dialog = gtk_color_selection_dialog_new("Font Color");

  result = gtk_dialog_run(GTK_DIALOG(dialog));

  if (result == GTK_RESPONSE_OK) {

    GdkColor color;

    colorsel = GTK_COLOR_SELECTION(
                   GTK_COLOR_SELECTION_DIALOG(dialog)->colorsel);
    gtk_color_selection_get_current_color(colorsel,
                                &color);

    gtk_widget_modify_fg(GTK_WIDGET(label),
                             GTK_STATE_NORMAL,
                             &color);
  } 

  gtk_widget_destroy(dialog);
}

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *widget;
  GtkWidget *label;
  GtkWidget *vbox;

  GtkWidget *toolbar;
  GtkToolItem *font;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 280, 200);
  gtk_window_set_title(GTK_WINDOW(window), "Color Selection Dialog");

  vbox = gtk_vbox_new(FALSE, 0);
  gtk_container_add(GTK_CONTAINER(window), vbox);

  toolbar = gtk_toolbar_new();
  gtk_toolbar_set_style(GTK_TOOLBAR(toolbar), GTK_TOOLBAR_ICONS);

  gtk_container_set_border_width(GTK_CONTAINER(toolbar), 2);

  font = gtk_tool_button_new_from_stock(GTK_STOCK_SELECT_COLOR);
  gtk_toolbar_insert(GTK_TOOLBAR(toolbar), font, -1);

  gtk_box_pack_start(GTK_BOX(vbox), toolbar, FALSE, FALSE, 5);

  label = gtk_label_new("ZetCode");
  gtk_label_set_justify(GTK_LABEL(label), GTK_JUSTIFY_CENTER);
  gtk_box_pack_start(GTK_BOX(vbox), label, TRUE, FALSE, 5);

  g_signal_connect(G_OBJECT(font), "clicked", 
        G_CALLBACK(select_font), label);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

The example is very similar to the previous one. This time we change the colour of the label.

```cpp
GtkWidget *dialog = gtk_color_selection_dialog_new("Font Color");
result = gtk_dialog_run(GTK_DIALOG(dialog));
```

We create and show the GtkColorSelectionDialog.

```cpp
if (result == GTK_RESPONSE_OK) {

  GdkColor color;

  colorsel = GTK_COLOR_SELECTION(
                 GTK_COLOR_SELECTION_DIALOG(dialog)->colorsel);
  gtk_color_selection_get_current_color(colorsel,
                 &color);

  gtk_widget_modify_fg(GTK_WIDGET(label),
                 GTK_STATE_NORMAL,
                 &color);
} 
```

If we press the OK button, we get the colour and modify the label's colour. The colour value is returned with the gtk_color_selection_get_current_color() function.

[Figure: GtkColorSelectionDialog]()

This chapter was about dialogs in GTK+. 