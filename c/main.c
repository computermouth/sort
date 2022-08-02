#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	void * prnt;
	void * left;
	void * rite;
	char * line;
} node_t;

int nodesort(const void * p1, const void * p2){
	return strcmp(p1, p2);
}

int main(){
	
	char * line = NULL;
	size_t len = 0;
	
	// first node
	node_t * root = calloc(1, sizeof(node_t));
	node_t * head = root;
	if (root != NULL && getline(&line, &len, stdin) != -1) {
		root->line = line;
		line = NULL;
		len = 0;
	} else {
		if (root != NULL) free(root);
		return 1;
	}
	
	// populate nodes
	while (getline(&line, &len, stdin) != -1){
		
		while (1) { // traverse tree
			int rc = strcmp(line, head->line);
			node_t *  next   = NULL;
			node_t ** next_p = NULL;
			
			// determine direction
			if (rc <= 0){
				next = head->left;
				next_p = (node_t **)&head->left;
			} else {
				next = head->rite;
				next_p = (node_t **)&head->rite;
			}
			
			// write out if unoccupied
			if (next == NULL) {
				next = calloc(1, sizeof(node_t));
				next->prnt = head;
				*next_p = next;
				next->line = line;
				
				head = root;
				line = NULL;
				len  = 0;
				break;
			}
			head = next;
			
		};
		
		head = root;
		line = NULL;
		len = 0;
	}
	
	free(line);
	
	while (head->left != NULL)
		head = head->left;
	
	while (root != NULL) {
		// traverse left
		if (head->left != NULL){
			head = head->left;
			continue;
		}
		
		// print self
		if (head->line != NULL){
			fprintf(stdout, "%s", head->line);
			free(head->line);
			head->line = NULL;
			continue;
		// traverse right
		} else if (head->rite != NULL) {
			head = head->rite;
			continue;
		}
		
		if (head == root)
			break;
		
		node_t * prnt = head->prnt;
		if (prnt->left == head)
			prnt->left = NULL;
		else
			prnt->rite = NULL;
		free(head);
		head = prnt;
	}
	free(root);
	
	return 0;
}
