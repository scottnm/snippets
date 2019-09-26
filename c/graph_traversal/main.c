// A quick DFS and BFS impl
//
// Graph looks like...
//
// A -> B,C
// B -> D,E
// C -> G,F
// D -> G
// E -> F
// F -> E
// A -> E == A->B->D->E || A->C->F->E
#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct node node;
struct node
{
    char val;
    size_t child_count;
    node** children;
    bool visited;
};

// constraints:
// for simplicity, node values can only be capital letters between A-Z
#define MAX_NODES ('Z' - 'A' + 1)
#define ARRAYSIZE(arr) (sizeof(arr) / sizeof(arr[0]))
#define NODE_AT(c) (&node_table[(c) - 'A'])

char*
find_delimiter(
    char* value_entry
    )
{
    char* entry = value_entry;
    while (*entry != '|') entry += 1;
    return entry;
}

void
fill_graph(
    node* node_table,
    char* value_entry
    )
{
    node* value_node = NODE_AT(*value_entry);
    char*const children_values = value_entry + 1;

    for (size_t child_index = 0; child_index < value_node->child_count; child_index += 1)
    {
        value_node->children[child_index] = NODE_AT(children_values[child_index]);
    }
}

bool
dfs_helper(
    node* start,
    node* end
   )
{
    if (start == end)
    {
        return true;
    }

    if (start->visited)
    {
        return false;
    }

    start->visited = true;
    for (size_t i = 0; i < start->child_count; i += 1)
    {
        if (dfs_helper(start->children[i], end))
        {
            start->visited = false;
            return true;
        }
    }

    start->visited = false;
    return false;
}

void
dfs(
    node* start,
    node* end
   )
{
    bool found = dfs_helper(start, end);
    printf("%c -> %c: %sFOUND\n", start->val, end->val, found ? "" : "NOT ");
}

int main()
{
    // hardcoded for simplicity but could be specified at runtime
    // format of node spec is...
    //    node groups are end with '|'
    //    each node group must have at least one value
    //    the first value in the node group is the node
    //    all subsequent values are the nodes children
    char node_spec[] =
    {
        'A', 'B', 'C', '|',
        'B', 'D', 'E', '|',
        'C', 'G', 'F', '|',
        'D', 'G', '|',
        'E', 'F', '|',
        'F', 'E', '|',
    };

    // Initialize the node table
    node node_table[MAX_NODES];
    for (char v = 'A'; v <= 'Z'; v += 1)
    {
        node* n = NODE_AT(v);
        n->val = v;
        n->child_count = 0;
        n->children = NULL;
        n->visited = false;
    }

    // setup graph
    for (char* value_entry = &node_spec[0]; value_entry < &node_spec[ARRAYSIZE(node_spec)]; /* no adv */)
    {
        char* delimiter_entry = find_delimiter(value_entry);
        assert(delimiter_entry > value_entry);
        size_t child_count = delimiter_entry - value_entry - 1;
        node* node = NODE_AT(*value_entry);
        assert(node->val == *value_entry);
        assert(node->child_count == 0);
        assert(node->children == NULL);
        node->child_count = child_count;
        node->children = calloc(node->child_count, sizeof(*node->children));
        fill_graph(node_table, value_entry);
        value_entry = delimiter_entry + 1;
    }

    printf("ALL:\n");
    for (char s = 'A'; s <= 'G'; s += 1)
    {
        for (char e = 'A'; e <= 'G'; e += 1)
        {
            dfs(NODE_AT(s), NODE_AT(e));
        }
    }
    printf("\n\n");
    printf("SAMPLE:\n");
    dfs(NODE_AT('A'), NODE_AT('E'));
    dfs(NODE_AT('B'), NODE_AT('A'));
    dfs(NODE_AT('E'), NODE_AT('E'));
    dfs(NODE_AT('E'), NODE_AT('F'));

    // teardown graph
    for (node* n = NODE_AT('A'); n <= NODE_AT('Z'); n += 1)
    {
        if (n->children != NULL)
        {
            free(n->children);
        }
    }
}
