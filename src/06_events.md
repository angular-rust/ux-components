# GTK+ events and signals

In this part of the GTK+ programming tutorial, we talk about the event system.

GTK+ is an event driven system. All GUI applications are event driven. The applications start a main loop, which continuously checks for newly generated events. If there is no event, the application waits and does nothing. In GTK+ an event is a message from the X server. When the event reaches a widget, it may react to this event by emitting a signal. The GTK+ programmer can connect a specific callback to the signal. The callback is a handler function that reacts to the signal.

## Button click

When a button is fired, it sends a clicked signal. A button can be fired by a mouse pointer or with the Space key (provided the button has focus).
buttonclick.c

```cpp
#include <gtk/gtk.h>

void button_clicked(GtkWidget *widget, gpointer data) {
    
  g_print("clicked\n");
}

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *halign;
  GtkWidget *btn;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_title(GTK_WINDOW(window), "GtkButton");
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);
  gtk_container_set_border_width(GTK_CONTAINER(window), 15);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);

  halign = gtk_alignment_new(0, 0, 0, 0);
  btn = gtk_button_new_with_label("Click");
  gtk_widget_set_size_request(btn, 70, 30);
  
  gtk_container_add(GTK_CONTAINER(halign), btn);
  gtk_container_add(GTK_CONTAINER(window), halign);

  g_signal_connect(G_OBJECT(btn), "clicked", 
      G_CALLBACK(button_clicked), NULL);

  g_signal_connect(G_OBJECT(window), "destroy", 
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In the application, we have two signals: the clicked signal and the destroy signal.

```cpp
g_signal_connect(G_OBJECT(btn), "clicked", 
    G_CALLBACK(button_clicked), NULL);
```

We use the g_signal_connect() function to connect the clicked signal to the button_clicked() callback.

```cpp
void button_clicked(GtkWidget *widget, gpointer data) {
    
  g_print("clicked\n");
}
```

The callback prints the "clicked" string to the console. The first parameter of the callback function is the object which emitted the signal. In our case it is the Click button. The second parameter is optional. We may send some data to the callback. In our case, we did not send any data; we provided a NULL value to the fourth parameter of the g_signal_connect() function.

```cpp
g_signal_connect(G_OBJECT(window), "destroy", 
     G_CALLBACK(gtk_main_quit), NULL);
```

If we press on the x button located in the upper right corner of the titlebar, or we press Atl+F4, a destroy signal is emitted. The gtk_main_quit() function is called, which terminates the application.

## Moving a window

The next example shows how we react to window move events.
moveevent.c

```cpp
#include <gtk/gtk.h>

void configure_callback(GtkWindow *window, 
      GdkEvent *event, gpointer data) {
          
   int x, y;
   GString *buf;
   
   x = event->configure.x;
   y = event->configure.y;
   
   buf = g_string_new(NULL);   
   g_string_printf(buf, "%d, %d", x, y);
   
   gtk_window_set_title(window, buf->str);
   
   g_string_free(buf, TRUE);
}

int main(int argc, char *argv[]) {
    
  GtkWidget *window;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);
  gtk_widget_add_events(GTK_WIDGET(window), GDK_CONFIGURE);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), G_OBJECT(window));

  g_signal_connect(G_OBJECT(window), "configure-event",
        G_CALLBACK(configure_callback), NULL);

  gtk_widget_show(window);
  gtk_main();

  return 0;
}
```

In the example, we show the current position of the upper-left corner of our window in the titlebar.

```cpp
gtk_widget_add_events(GTK_WIDGET(window), GDK_CONFIGURE);
```

The event mask of the widget determines what kind of events will a particular widget receive. Some event are preconfigured, other events have to be added to the event mask. The gtk_widget_add_events() adds a GDK_CONFIGURE event type to the mask. The GDK_CONFIGURE event type accounts for all size, position, and the stacking order of the window changes.

```cpp
g_signal_connect(G_OBJECT(window), "configure-event",
    G_CALLBACK(configure_callback), NULL);
```

The configure-event is emitted when the size, position, or stacking of the widget's window has changed.

```cpp
void configure_callback(GtkWindow *window, 
      GdkEvent *event, gpointer data) {
          
   int x, y;
   GString *buf;
   
   x = event->configure.x;
   y = event->configure.y;
   
   buf = g_string_new(NULL);   
   g_string_printf(buf, "%d, %d", x, y);
   
   gtk_window_set_title(window, buf->str);
   
   g_string_free(buf, TRUE);
}
```

The callback function has three parameters: the object that emitted the signal, the GdkEvent, and the optional data. We determine the x, y coordinates, build a string, and set it to the window title.

[Figure: Move event]()

## The enter signal

The following example shows how we can react to an enter signal. The enter signal is emitted when we enter the area of a widget with a mouse pointer.
entersignal.c

```cpp
#include <gtk/gtk.h>

void enter_button(GtkWidget *widget, gpointer data) {
     
  GdkColor col = {0, 27000, 30000, 35000};   
  
  gtk_widget_modify_bg(widget, GTK_STATE_PRELIGHT, &col);
}

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *halign;
  GtkWidget *btn;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);
  gtk_container_set_border_width(GTK_CONTAINER(window), 15);
  gtk_window_set_title(GTK_WINDOW(window), "Enter signal");

  halign = gtk_alignment_new(0, 0, 0, 0);

  btn = gtk_button_new_with_label("Button");
  gtk_widget_set_size_request(btn, 70, 30);
  
  gtk_container_add(GTK_CONTAINER(halign), btn);
  gtk_container_add(GTK_CONTAINER(window), halign);

  g_signal_connect(G_OBJECT(btn), "enter", 
      G_CALLBACK(enter_button), NULL);

  g_signal_connect(G_OBJECT(window), "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In the example, the background colour of the button widget is changes when we hover a mouse pointer over it.

```cpp
g_signal_connect(G_OBJECT(btn), "enter", 
    G_CALLBACK(enter_button), NULL);
```

We call the enter_button() user function when the enter signal occurs.

```cpp
void enter_button(GtkWidget *widget, gpointer data) {
     
  GdkColor col = {0, 27000, 30000, 35000};   
  
  gtk_widget_modify_bg(widget, GTK_STATE_PRELIGHT, &col);
}
```

Inside the callback, we change the background of the button by calling the gtk_widget_modify_bg() function.

## Disconnecting a callback

We can disconnect a callback from the signal. The next code example demonstrates such a case.
disconnect.c

```cpp
#include <gtk/gtk.h>

gint handler_id;

void button_clicked(GtkWidget *widget, gpointer data) {
     
  g_print("clicked\n");
}

void toogle_signal(GtkWidget *widget, gpointer window) {
    
  if (gtk_toggle_button_get_active(GTK_TOGGLE_BUTTON(widget))) {
     handler_id = g_signal_connect(G_OBJECT(window), "clicked", 
           G_CALLBACK(button_clicked), NULL);
  } else {
     g_signal_handler_disconnect(window, handler_id);
  }
}

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *hbox;
  GtkWidget *vbox;
  GtkWidget *btn;
  GtkWidget *cb;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);
  gtk_container_set_border_width(GTK_CONTAINER(window), 15);
  gtk_window_set_title(GTK_WINDOW(window), "Disconnect");

  hbox = gtk_hbox_new(FALSE, 15);

  btn = gtk_button_new_with_label("Click");
  gtk_widget_set_size_request(btn, 70, 30);
  gtk_box_pack_start(GTK_BOX(hbox), btn, FALSE, FALSE, 0);

  cb = gtk_check_button_new_with_label("Connect");
  gtk_toggle_button_set_active(GTK_TOGGLE_BUTTON(cb), TRUE);
  gtk_box_pack_start(GTK_BOX(hbox), cb, FALSE, FALSE, 0);
  
  vbox = gtk_vbox_new(FALSE, 5);
  gtk_box_pack_start(GTK_BOX(vbox), hbox, FALSE, FALSE, 0);
  gtk_container_add(GTK_CONTAINER(window), vbox);  

  handler_id = g_signal_connect(G_OBJECT(btn), "clicked", 
      G_CALLBACK(button_clicked), NULL);

  g_signal_connect(G_OBJECT(cb), "clicked",
      G_CALLBACK(toogle_signal), (gpointer) btn);

  g_signal_connect(G_OBJECT(window), "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In the code example, we have a button and a check box. The check box connects or disconnects a callback from the clicked signal of the button.

```cpp
handler_id = g_signal_connect(G_OBJECT(btn), "clicked", 
    G_CALLBACK(button_clicked), NULL);
```

The g_signal_connect() returns the handler id which uniquely identifies the callback.

```cpp
if (gtk_toggle_button_get_active(GTK_TOGGLE_BUTTON(widget))) {
   handler_id = g_signal_connect(G_OBJECT(window), "clicked", 
         G_CALLBACK(button_clicked), NULL);
} else {
   g_signal_handler_disconnect(window, handler_id);
}
```

This code determines the state of the check box. Depending on the state, it connects the callback with the g_signal_connect() function or disconnects with the g_signal_handler_disconnect() function.

[Figure: Disconnect]()

## Drag and drop example

In the next example, we show borderless window and learn how we can drag and move such a window.
dragdrop.c

```cpp
#include <gtk/gtk.h>

gboolean on_button_press(GtkWidget* widget,
  GdkEventButton *event, GdkWindowEdge edge) {
      
  if (event->type == GDK_BUTTON_PRESS) {
      
    if (event->button == 1) {
      gtk_window_begin_move_drag(GTK_WINDOW(gtk_widget_get_toplevel(widget)),
          event->button,
          event->x_root,
          event->y_root,
          event->time);
    }
  }
  
  return TRUE;
}

int main(int argc, char *argv[]) {

  GtkWidget *window;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 250, 200);
  gtk_window_set_title(GTK_WINDOW(window), "Drag & drop");
  gtk_window_set_decorated(GTK_WINDOW(window), FALSE);
  gtk_widget_add_events(window, GDK_BUTTON_PRESS_MASK);

  g_signal_connect(G_OBJECT(window), "button-press-event",
      G_CALLBACK(on_button_press), NULL);

  g_signal_connect(G_OBJECT(window), "destroy",
        G_CALLBACK(gtk_main_quit), G_OBJECT(window));

  gtk_widget_show(window);

  gtk_main();

  return 0;
}
```

The example demonstrates a drag and drop operation of a borderless window.

```cpp
gtk_window_set_decorated(GTK_WINDOW(window), FALSE);
```

We remove the window decorations with the gtk_window_set_decorated() function. This means that the window will not have borders and titlebar.

```cpp
g_signal_connect(G_OBJECT(window), "button-press-event",
    G_CALLBACK(on_button_press), NULL);
```

We connect the window to the button-press-event signal.

```cpp
gboolean on_button_press(GtkWidget* widget,
  GdkEventButton *event, GdkWindowEdge edge) {
      
  if (event->type == GDK_BUTTON_PRESS) {
      
    if (event->button == 1) {
      gtk_window_begin_move_drag(GTK_WINDOW(gtk_widget_get_toplevel(widget)),
          event->button,
          event->x_root,
          event->y_root,
          event->time);
    }
  }
  
  return TRUE;
}
```

Inside the on_button_press() function, we perform the drag and drop operation. We check if the left mouse button was pressed. Then we call the gtk_window_begin_move_drag() function, which starts moving the window.

## A timer example

The following example demonstrates a timer example. Timers are used when we have some repeating tasks. It could be a clock, a count down, visual effects, or animations.
timer.c

```cpp
#include <cairo.h>
#include <gtk/gtk.h>

gchar buf[256];

gboolean on_expose_event(GtkWidget *widget,
    GdkEventExpose *event,
    gpointer data) {
        
  cairo_t *cr;

  cr = gdk_cairo_create(widget->window);

  cairo_move_to(cr, 30, 30);
  cairo_set_font_size(cr, 15);
  cairo_show_text(cr, buf);

  cairo_destroy(cr);

  return FALSE;
}

gboolean time_handler(GtkWidget *widget) {
    
  if (widget->window == NULL) return FALSE;

  GDateTime *now = g_date_time_new_now_local(); 
  gchar *my_time = g_date_time_format(now, "%H:%M:%S");
  
  g_sprintf(buf, "%s", my_time);
  
  g_free(my_time);
  g_date_time_unref(now);

  gtk_widget_queue_draw(widget);
  
  return TRUE;
}

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *darea;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  darea = gtk_drawing_area_new();
  gtk_container_add(GTK_CONTAINER(window), darea);

  g_signal_connect(darea, "expose-event",
      G_CALLBACK(on_expose_event), NULL);
  g_signal_connect(window, "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);

  gtk_window_set_title(GTK_WINDOW(window), "Timer");
  g_timeout_add(1000, (GSourceFunc) time_handler, (gpointer) window);
  gtk_widget_show_all(window);
  time_handler(window);

  gtk_main();

  return 0;
}
```

The example displays the current local time on the window. The Cairo 2D library is also used.

```cpp
g_signal_connect(darea, "expose-event",
    G_CALLBACK(on_expose_event), NULL);
```

We draw the time inside the on_expose_event() callback. The callback is connected to the expose-event signal, which is is emitted when the window is going to be redrawn.

```cpp
g_timeout_add(1000, (GSourceFunc) time_handler, (gpointer) window);
```

This function registers the timer. The time_handler() function is called repeatedly at regular intervals; in our case in every second. The timer function is called until it returns FALSE.

```cpp
time_handler(window);
```

This calls the timer function immediately. Otherwise, there would be one sec delay.

```cpp
cairo_t *cr;

cr = gdk_cairo_create(widget->window);

cairo_move_to(cr, 30, 30);
cairo_set_font_size(cr, 15);
cairo_show_text(cr, buf);

cairo_destroy(cr);
```

This code draws the current time on the window. For more information about the Cairo 2D library, see the ZetCode's Cairo graphics tutorial.

```cpp
if (widget->window == NULL) return FALSE;
```
When the window is destroyed, it may happen that the timer function is called. This line prevents working on an already destroyed widget.

```cpp
GDateTime *now = g_date_time_new_now_local(); 
gchar *my_time = g_date_time_format(now, "%H:%M:%S");

g_sprintf(buf, "%s", my_time);
```

These lines determine the current local time. The time is stored in the global buf variable.

```cpp
gtk_widget_queue_draw(widget);
```

The gtk_widget_queue_draw() function invalidates the window area, which then emits the expose-event signal.

This chapter was about events in GTK+. 