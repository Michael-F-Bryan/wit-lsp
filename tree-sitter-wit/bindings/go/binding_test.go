package tree_sitter_WIT_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-WIT"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_WIT.Language())
	if language == nil {
		t.Errorf("Error loading Wit grammar")
	}
}
