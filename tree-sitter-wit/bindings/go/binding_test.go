package tree_sitter_wit_test

import (
	"testing"

	tree_sitter_wit "github.com/Michael-F-Bryan/wit-lsp/tree-sitter-wit"
	tree_sitter "github.com/smacker/go-tree-sitter"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_wit.Language())
	if language == nil {
		t.Errorf("Error loading Wit grammar")
	}
}
