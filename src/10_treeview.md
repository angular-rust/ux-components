# GtkTreeView widget

In this part of the GTK+ programming tutorial, we work with the GtkTreeView widget.

GtkTreeView widget is a complex widget which can be used to display lists and trees. The widget can have one or multiple columns. The GtkTreeView widget has a MVC (Model View Controller) design architecture. This means that the data is separated from the view.

There are several other objects that are used with the GtkTreeView widget. The GtkCellRenderer determines how the data is going to be displayed in the GtkTreeViewColumn. The GtkListStore and the GtkTreeStore represent the model. They handle data that are displayed in the GtkTreeView widget. GtkTreeIter is a structure used to refer to a row in the GtkTreeView. GtkTreeSelection is an object that handles selections.

## List view

The first example will show a simple list view. We will display textual data.
listview.c

```cpp
#include <gtk/gtk.h>

enum {

  LIST_ITEM = 0,
  N_COLUMNS
};

void init_list(GtkWidget *list) {

  GtkCellRenderer *renderer;
  GtkTreeViewColumn *column;
  GtkListStore *store;

  renderer = gtk_cell_renderer_text_new ();
  column = gtk_tree_view_column_new_with_attributes("List Items",
          renderer, "text", LIST_ITEM, NULL);
  gtk_tree_view_append_column(GTK_TREE_VIEW(list), column);

  store = gtk_list_store_new(N_COLUMNS, G_TYPE_STRING);

  gtk_tree_view_set_model(GTK_TREE_VIEW(list), 
      GTK_TREE_MODEL(store));

  g_object_unref(store);
}

void add_to_list(GtkWidget *list, const gchar *str) {
    
  GtkListStore *store;
  GtkTreeIter iter;

  store = GTK_LIST_STORE(gtk_tree_view_get_model
      (GTK_TREE_VIEW(list)));

  gtk_list_store_append(store, &iter);
  gtk_list_store_set(store, &iter, LIST_ITEM, str, -1);
}

void on_changed(GtkWidget *widget, gpointer label) {
    
  GtkTreeIter iter;
  GtkTreeModel *model;
  gchar *value;

  if (gtk_tree_selection_get_selected(
      GTK_TREE_SELECTION(widget), &model, &iter)) {

    gtk_tree_model_get(model, &iter, LIST_ITEM, &value,  -1);
    gtk_label_set_text(GTK_LABEL(label), value);
    g_free(value);
  }
}

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *list;

  GtkWidget *vbox;
  GtkWidget *label;
  GtkTreeSelection *selection; 

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  list = gtk_tree_view_new();

  gtk_window_set_title(GTK_WINDOW(window), "List view");
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_container_set_border_width(GTK_CONTAINER(window), 10);
  gtk_window_set_default_size(GTK_WINDOW(window), 270, 250);

  gtk_tree_view_set_headers_visible(GTK_TREE_VIEW(list), FALSE);

  vbox = gtk_vbox_new(FALSE, 0);

  gtk_box_pack_start(GTK_BOX(vbox), list, TRUE, TRUE, 5);

  label = gtk_label_new("");
  gtk_box_pack_start(GTK_BOX(vbox), label, FALSE, FALSE, 5);

  gtk_container_add(GTK_CONTAINER(window), vbox);

  init_list(list);
  add_to_list(list, "Aliens");
  add_to_list(list, "Leon");
  add_to_list(list, "The Verdict");
  add_to_list(list, "North Face");
  add_to_list(list, "Der Untergang");

  selection = gtk_tree_view_get_selection(GTK_TREE_VIEW(list));

  g_signal_connect(selection, "changed", 
      G_CALLBACK(on_changed), label);

  g_signal_connect(G_OBJECT (window), "destroy",
      G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In our code example, we show five items in the GtkTreeView. We have only one column and the header of the column is hidden. We place a GtkVBox into the window. This box has two widgets: a GtkTreeView and a GtkLabel.

```cpp
list = gtk_tree_view_new();
```

The gtk_tree_view_new() function creates a new GtkTreeView widget.

```cpp
gtk_tree_view_set_headers_visible(GTK_TREE_VIEW(list), FALSE);
```

We hide the column header with the gtk_tree_view_set_headers_visible() function.

```cpp
label = gtk_label_new("");
gtk_box_pack_start(GTK_BOX(vbox), label, FALSE, FALSE, 5);
```

The GtkLabel is created and placed below the GtkTreeView.

```cpp
init_list(list);
```

This function initializes the list.

```cpp
renderer = gtk_cell_renderer_text_new();
column = gtk_tree_view_column_new_with_attributes("List Items",
        renderer, "text", LIST_ITEM, NULL);
gtk_tree_view_append_column(GTK_TREE_VIEW(list), column);
```

Inside that function, we create and append one single column.

```cpp
store = gtk_list_store_new(N_COLUMNS, G_TYPE_STRING);

gtk_tree_view_set_model(GTK_TREE_VIEW(list), 
    GTK_TREE_MODEL(store));
```

We create a GtkListStore (a model) and set it to the list.

```cpp
g_object_unref(store);
```

The TreeView increases the reference of the store object. We decrease the reference from 2 to 1 with the g_object_unref() function. The model is then destroyed automatically with the view.

```cpp
add_to_list(list, "Aliens");
```

This user function adds an option to the list.

```cpp
store = GTK_LIST_STORE(gtk_tree_view_get_model
    (GTK_TREE_VIEW(list)));

gtk_list_store_append(store, &iter);
gtk_list_store_set(store, &iter, LIST_ITEM, str, -1);
```

Inside the add_to_list() function, we get the model using the gtk_tree_view_get_model() function call. We append a new row and set a value to the row, which is referenced by an GtkTreeIter object.

```cpp
selection = gtk_tree_view_get_selection(GTK_TREE_VIEW(list));
```

The GtkTreeSelection does not need to be created explicitly; it is automatically created with the GtkTreeView widget. The reference to the widget is obtained using the gtk_tree_view_get_selection() function call.

```cpp
g_signal_connect(selection, "changed", 
    G_CALLBACK(on_changed), label);
```

The changed signal of the GtkTreeSelection is connected to the on_changed() handler.

```cpp
if (gtk_tree_selection_get_selected(
    GTK_TREE_SELECTION(widget), &model, &iter)) {
```

The gtk_tree_selection_get_selected() function sets the iter to the currently selected node.

```cpp
gtk_tree_model_get(model, &iter, LIST_ITEM, &value,  -1);
```

Inside the handler function, we get the value of the cell in the row referenced by the iter object.

```cpp
gtk_label_set_text(GTK_LABEL(label), value);
```
The retrieved value is set to the label with the gtk_label_set_text() function.

[Figure: List view]()

## Dynamic List view

The second example adds additional functionality to the previous one. We will be able to add and remove items from the list view.
dynamiclistview.c

```cpp
#include <gtk/gtk.h>

enum {
    
  LIST_ITEM = 0,
  N_COLUMNS
};

GtkWidget *list;

void append_item(GtkWidget *widget, gpointer entry) {
    
  GtkListStore *store;
  GtkTreeIter iter;

  const gchar *str = gtk_entry_get_text(entry); 

  store = GTK_LIST_STORE(gtk_tree_view_get_model(GTK_TREE_VIEW(list)));

  gtk_list_store_append(store, &iter);
  gtk_list_store_set(store, &iter, LIST_ITEM, str, -1);
  
  gtk_entry_set_text(entry, "");
}

void remove_item(GtkWidget *widget, gpointer selection) {
    
  GtkListStore *store;
  GtkTreeModel *model;
  GtkTreeIter  iter;

  store = GTK_LIST_STORE(gtk_tree_view_get_model(GTK_TREE_VIEW(list)));
  model = gtk_tree_view_get_model(GTK_TREE_VIEW(list));

  if (gtk_tree_model_get_iter_first(model, &iter) == FALSE) {
      return;
  }

  if (gtk_tree_selection_get_selected(GTK_TREE_SELECTION(selection), 
         &model, &iter)) {
    gtk_list_store_remove(store, &iter);
  }
}

void remove_all(GtkWidget *widget, gpointer selection) {
    
  GtkListStore *store;
  GtkTreeModel *model;
  GtkTreeIter  iter;

  store = GTK_LIST_STORE(gtk_tree_view_get_model(GTK_TREE_VIEW(list)));
  model = gtk_tree_view_get_model(GTK_TREE_VIEW(list));

  if (gtk_tree_model_get_iter_first(model, &iter) == FALSE) {
      return;
  }
  
  gtk_list_store_clear(store);
}

void init_list(GtkWidget *list) {

  GtkCellRenderer    *renderer;
  GtkTreeViewColumn  *column;
  GtkListStore       *store;

  renderer = gtk_cell_renderer_text_new();
  column = gtk_tree_view_column_new_with_attributes("List Item",
          renderer, "text", LIST_ITEM, NULL);
  gtk_tree_view_append_column(GTK_TREE_VIEW(list), column);

  store = gtk_list_store_new(N_COLUMNS, G_TYPE_STRING);

  gtk_tree_view_set_model(GTK_TREE_VIEW(list), GTK_TREE_MODEL(store));

  g_object_unref(store);
}

int main(int argc, char *argv[]) {

  GtkWidget *window;
  GtkWidget *sw;

  GtkWidget *remove;
  GtkWidget *add;
  GtkWidget *removeAll;
  GtkWidget *entry;

  GtkWidget *vbox;
  GtkWidget *hbox;

  GtkTreeSelection *selection; 

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

  gtk_window_set_title(GTK_WINDOW(window), "List view");
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_container_set_border_width(GTK_CONTAINER (window), 10);
  gtk_widget_set_size_request(window, 370, 270);
  
  sw = gtk_scrolled_window_new(NULL, NULL);
  list = gtk_tree_view_new();  
  gtk_container_add(GTK_CONTAINER(sw), list);

  gtk_scrolled_window_set_policy(GTK_SCROLLED_WINDOW(sw),
            GTK_POLICY_AUTOMATIC, GTK_POLICY_AUTOMATIC);

  gtk_scrolled_window_set_shadow_type(GTK_SCROLLED_WINDOW(sw),
            GTK_SHADOW_ETCHED_IN);

  gtk_tree_view_set_headers_visible(GTK_TREE_VIEW(list), FALSE);

  vbox = gtk_vbox_new(FALSE, 0);

  gtk_box_pack_start(GTK_BOX(vbox), sw, TRUE, TRUE, 5);

  hbox = gtk_hbox_new(FALSE, 5);

  add = gtk_button_new_with_label("Add");
  remove = gtk_button_new_with_label("Remove");
  removeAll = gtk_button_new_with_label("Remove All");
  entry = gtk_entry_new();
  gtk_widget_set_size_request(entry, 120, -1);

  gtk_box_pack_start(GTK_BOX(hbox), add, FALSE, TRUE, 3);
  gtk_box_pack_start(GTK_BOX(hbox), entry, FALSE, TRUE, 3);
  gtk_box_pack_start(GTK_BOX(hbox), remove, FALSE, TRUE, 3);
  gtk_box_pack_start(GTK_BOX(hbox), removeAll, FALSE, TRUE, 3);

  gtk_box_pack_start(GTK_BOX(vbox), hbox, FALSE, TRUE, 3);

  gtk_container_add(GTK_CONTAINER(window), vbox);

  init_list(list);

  selection = gtk_tree_view_get_selection(GTK_TREE_VIEW(list));

  g_signal_connect(G_OBJECT(add), "clicked",
          G_CALLBACK(append_item), entry);

  g_signal_connect(G_OBJECT(remove), "clicked",
          G_CALLBACK(remove_item), selection);

  g_signal_connect(G_OBJECT(removeAll), "clicked",
          G_CALLBACK(remove_all), selection);

  g_signal_connect(G_OBJECT(window), "destroy",
          G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In the example, we have three buttons and one text entry. The buttons add a new item, remove the selected item, and remove all items.

```cpp
sw = gtk_scrolled_window_new(NULL, NULL);
list = gtk_tree_view_new();  
gtk_container_add(GTK_CONTAINER(sw), list);
```

The GtkTreeView is placed inside a scrolled window.

```cpp
if (gtk_tree_selection_get_selected(GTK_TREE_SELECTION(selection), 
    &model, &iter)) {
  gtk_list_store_remove(store, &iter);
}
```

The gtk_list_store_remove() function removes an item from the list.

```cpp
gtk_list_store_clear(store);
```

The gtk_list_store_clear() removes all items from the list.

```cpp
if (gtk_tree_model_get_iter_first(model, &iter) == FALSE) {
    return;
}
```

This code checks if there is some item left in the list. Obviously, we can remove items only if there is at least one left in the list.

[Figure: Dynamic List view]()

## Tree view

The following example uses the GtkTreeView widget to display hierarchical data. In the previous two examples, we used the list view; now we are going to use tree view.
treeview.c

```cpp
#include <gtk/gtk.h>

enum {
  COLUMN = 0,
  NUM_COLS
};

void on_changed(GtkWidget *widget, gpointer statusbar) {
    
  GtkTreeIter iter;
  GtkTreeModel *model;
  gchar *value;

  if (gtk_tree_selection_get_selected(
      GTK_TREE_SELECTION(widget), &model, &iter)) {

    gtk_tree_model_get(model, &iter, COLUMN, &value,  -1);
    gtk_statusbar_push(GTK_STATUSBAR(statusbar),
        gtk_statusbar_get_context_id(GTK_STATUSBAR(statusbar), 
            value), value);
    g_free(value);
  }
}

GtkTreeModel *create_and_fill_model(void) {
    
  GtkTreeStore *treestore;
  GtkTreeIter toplevel, child;

  treestore = gtk_tree_store_new(NUM_COLS,
                  G_TYPE_STRING);

  gtk_tree_store_append(treestore, &toplevel, NULL);
  gtk_tree_store_set(treestore, &toplevel,
                     COLUMN, "Scripting languages",
                     -1);

  gtk_tree_store_append(treestore, &child, &toplevel);
  gtk_tree_store_set(treestore, &child,
                     COLUMN, "Python",
                     -1);
  gtk_tree_store_append(treestore, &child, &toplevel);
  gtk_tree_store_set(treestore, &child,
                     COLUMN, "Perl",
                     -1);
  gtk_tree_store_append(treestore, &child, &toplevel);
  gtk_tree_store_set(treestore, &child,
                     COLUMN, "PHP",
                     -1);

  gtk_tree_store_append(treestore, &toplevel, NULL);
  gtk_tree_store_set(treestore, &toplevel,
                     COLUMN, "Compiled languages",
                     -1);

  gtk_tree_store_append(treestore, &child, &toplevel);
  gtk_tree_store_set(treestore, &child,
                     COLUMN, "C",
                     -1);

  gtk_tree_store_append(treestore, &child, &toplevel);
  gtk_tree_store_set(treestore, &child,
                     COLUMN, "C++",
                     -1);

  gtk_tree_store_append(treestore, &child, &toplevel);
  gtk_tree_store_set(treestore, &child,
                     COLUMN, "Java",
                     -1);

  return GTK_TREE_MODEL(treestore);
}


GtkWidget *create_view_and_model(void) {
    
  GtkTreeViewColumn *col;
  GtkCellRenderer *renderer;
  GtkWidget *view;
  GtkTreeModel *model;

  view = gtk_tree_view_new();

  col = gtk_tree_view_column_new();
  gtk_tree_view_column_set_title(col, "Programming languages");
  gtk_tree_view_append_column(GTK_TREE_VIEW(view), col);

  renderer = gtk_cell_renderer_text_new();
  gtk_tree_view_column_pack_start(col, renderer, TRUE);
  gtk_tree_view_column_add_attribute(col, renderer, 
      "text", COLUMN);

  model = create_and_fill_model();
  gtk_tree_view_set_model(GTK_TREE_VIEW(view), model);
  g_object_unref(model); 

  return view;
}

int main(int argc, char *argv[]) {
    
  GtkWidget *window;
  GtkWidget *view;
  GtkTreeSelection *selection; 
  GtkWidget *vbox;
  GtkWidget *statusbar;

  gtk_init(&argc, &argv);

  window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
  gtk_window_set_title(GTK_WINDOW(window), "Tree view");
  gtk_widget_set_size_request(window, 350, 300);

  vbox = gtk_vbox_new(FALSE, 2);
  gtk_container_add(GTK_CONTAINER(window), vbox);

  view = create_view_and_model();
  selection = gtk_tree_view_get_selection(GTK_TREE_VIEW(view));

  gtk_box_pack_start(GTK_BOX(vbox), view, TRUE, TRUE, 1);

  statusbar = gtk_statusbar_new();
  gtk_box_pack_start(GTK_BOX(vbox), statusbar, FALSE, TRUE, 1);

  g_signal_connect(selection, "changed", 
      G_CALLBACK(on_changed), statusbar);

  g_signal_connect (G_OBJECT (window), "destroy",
          G_CALLBACK(gtk_main_quit), NULL);

  gtk_widget_show_all(window);

  gtk_main();

  return 0;
}
```

In the example, we divide programming languages into two groups: scripting languages and compiled languages. The language categories serve as toplevel nodes for their list of items. The currently selected item is shown in the statusbar. The steps to create a tree view are very similar to creating a list view.

```cpp
treestore = gtk_tree_store_new(NUM_COLS,
                G_TYPE_STRING);
```

The gtk_tree_store_new() function creates a GtkTreeStore, which is a tree-like data structure that is used with the GtkTreeView.

```cpp
gtk_tree_store_append(treestore, &toplevel, NULL);
gtk_tree_store_set(treestore, &toplevel,
                  COLUMN, "Scripting languages",
                  -1);
```

These two lines create a toplevel node.

```cpp
gtk_tree_store_append(treestore, &child, &toplevel);
gtk_tree_store_set(treestore, &child,
                  COLUMN, "Python",
                  -1);
```

Here we add a child item into the toplevel node.

[Figure: Tree View]()

In this chapter we covered the GtkTreeView widget. 