#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 34
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 22
#define ALIAS_COUNT 0
#define TOKEN_COUNT 12
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 4
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 4

enum ts_symbol_identifiers {
  sym_identifier = 1,
  anon_sym_package = 2,
  anon_sym_SEMI = 3,
  anon_sym_COLON = 4,
  anon_sym_AT = 5,
  anon_sym_SLASH = 6,
  anon_sym_use = 7,
  anon_sym_as = 8,
  sym_semver = 9,
  sym_block_comment = 10,
  sym_slash_comment = 11,
  sym_source_file = 12,
  sym_package_decl = 13,
  sym_fully_qualified_package_name = 14,
  sym_package_name = 15,
  sym_package_path = 16,
  sym_top_level_item = 17,
  sym_top_level_use_item = 18,
  sym_use_path = 19,
  aux_sym_source_file_repeat1 = 20,
  aux_sym_package_path_repeat1 = 21,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_package] = "package",
  [anon_sym_SEMI] = ";",
  [anon_sym_COLON] = ":",
  [anon_sym_AT] = "@",
  [anon_sym_SLASH] = "/",
  [anon_sym_use] = "use",
  [anon_sym_as] = "as",
  [sym_semver] = "semver",
  [sym_block_comment] = "block_comment",
  [sym_slash_comment] = "slash_comment",
  [sym_source_file] = "source_file",
  [sym_package_decl] = "package_decl",
  [sym_fully_qualified_package_name] = "fully_qualified_package_name",
  [sym_package_name] = "package_name",
  [sym_package_path] = "package_path",
  [sym_top_level_item] = "top_level_item",
  [sym_top_level_use_item] = "top_level_use_item",
  [sym_use_path] = "use_path",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_package_path_repeat1] = "package_path_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [anon_sym_package] = anon_sym_package,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_use] = anon_sym_use,
  [anon_sym_as] = anon_sym_as,
  [sym_semver] = sym_semver,
  [sym_block_comment] = sym_block_comment,
  [sym_slash_comment] = sym_slash_comment,
  [sym_source_file] = sym_source_file,
  [sym_package_decl] = sym_package_decl,
  [sym_fully_qualified_package_name] = sym_fully_qualified_package_name,
  [sym_package_name] = sym_package_name,
  [sym_package_path] = sym_package_path,
  [sym_top_level_item] = sym_top_level_item,
  [sym_top_level_use_item] = sym_top_level_use_item,
  [sym_use_path] = sym_use_path,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_package_path_repeat1] = aux_sym_package_path_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_package] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_use] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_as] = {
    .visible = true,
    .named = false,
  },
  [sym_semver] = {
    .visible = true,
    .named = true,
  },
  [sym_block_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_slash_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_package_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_fully_qualified_package_name] = {
    .visible = true,
    .named = true,
  },
  [sym_package_name] = {
    .visible = true,
    .named = true,
  },
  [sym_package_path] = {
    .visible = true,
    .named = true,
  },
  [sym_top_level_item] = {
    .visible = true,
    .named = true,
  },
  [sym_top_level_use_item] = {
    .visible = true,
    .named = true,
  },
  [sym_use_path] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_package_path_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_alias = 1,
  field_package = 2,
  field_path = 3,
  field_version = 4,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_alias] = "alias",
  [field_package] = "package",
  [field_path] = "path",
  [field_version] = "version",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 1},
  [3] = {.index = 3, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_package, 0},
    {field_path, 2},
  [2] =
    {field_alias, 3},
  [3] =
    {field_package, 0},
    {field_path, 2},
    {field_version, 4},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(17);
      if (lookahead == '/') ADVANCE(21);
      if (lookahead == '0') ADVANCE(29);
      if (lookahead == ':') ADVANCE(19);
      if (lookahead == ';') ADVANCE(18);
      if (lookahead == '@') ADVANCE(20);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(15)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(28);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(30);
      END_STATE();
    case 1:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead == '/') ADVANCE(32);
      END_STATE();
    case 2:
      if (lookahead == '*') ADVANCE(2);
      if (lookahead == '/') ADVANCE(31);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 3:
      if (lookahead == '*') ADVANCE(2);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 4:
      if (lookahead == '.') ADVANCE(9);
      END_STATE();
    case 5:
      if (lookahead == '.') ADVANCE(9);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(5);
      END_STATE();
    case 6:
      if (lookahead == '.') ADVANCE(10);
      END_STATE();
    case 7:
      if (lookahead == '.') ADVANCE(10);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(7);
      END_STATE();
    case 8:
      if (lookahead == '/') ADVANCE(1);
      if (lookahead == '0') ADVANCE(4);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(8)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(5);
      END_STATE();
    case 9:
      if (lookahead == '0') ADVANCE(6);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(7);
      END_STATE();
    case 10:
      if (lookahead == '0') ADVANCE(22);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(23);
      END_STATE();
    case 11:
      if (lookahead == '0') ADVANCE(24);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(25);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 12:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(12);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 13:
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(27);
      END_STATE();
    case 14:
      if (eof) ADVANCE(17);
      if (lookahead == '/') ADVANCE(21);
      if (lookahead == ':') ADVANCE(19);
      if (lookahead == ';') ADVANCE(18);
      if (lookahead == '@') ADVANCE(20);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(16)
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(30);
      END_STATE();
    case 15:
      if (eof) ADVANCE(17);
      if (lookahead == '/') ADVANCE(1);
      if (lookahead == '0') ADVANCE(29);
      if (lookahead == ';') ADVANCE(18);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(15)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(28);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(30);
      END_STATE();
    case 16:
      if (eof) ADVANCE(17);
      if (lookahead == '/') ADVANCE(1);
      if (lookahead == ';') ADVANCE(18);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(16)
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(30);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(3);
      if (lookahead == '/') ADVANCE(32);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '+') ADVANCE(13);
      if (lookahead == '-') ADVANCE(11);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '+') ADVANCE(13);
      if (lookahead == '-') ADVANCE(11);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(23);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '+') ADVANCE(13);
      if (lookahead == '.') ADVANCE(11);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(12);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '+') ADVANCE(13);
      if (lookahead == '.') ADVANCE(11);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '+') ADVANCE(13);
      if (lookahead == '.') ADVANCE(11);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '.') ADVANCE(13);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(27);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '.') ADVANCE(9);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(28);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(30);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '.') ADVANCE(9);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(30);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(30);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_block_comment);
      if (lookahead == '*') ADVANCE(2);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_slash_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(32);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (lookahead == 'a') ADVANCE(1);
      if (lookahead == 'p') ADVANCE(2);
      if (lookahead == 'u') ADVANCE(3);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == 's') ADVANCE(4);
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(5);
      END_STATE();
    case 3:
      if (lookahead == 's') ADVANCE(6);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_as);
      END_STATE();
    case 5:
      if (lookahead == 'c') ADVANCE(7);
      END_STATE();
    case 6:
      if (lookahead == 'e') ADVANCE(8);
      END_STATE();
    case 7:
      if (lookahead == 'k') ADVANCE(9);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 9:
      if (lookahead == 'a') ADVANCE(10);
      END_STATE();
    case 10:
      if (lookahead == 'g') ADVANCE(11);
      END_STATE();
    case 11:
      if (lookahead == 'e') ADVANCE(12);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_package);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 14},
  [2] = {.lex_state = 14},
  [3] = {.lex_state = 14},
  [4] = {.lex_state = 14},
  [5] = {.lex_state = 14},
  [6] = {.lex_state = 14},
  [7] = {.lex_state = 14},
  [8] = {.lex_state = 14},
  [9] = {.lex_state = 14},
  [10] = {.lex_state = 14},
  [11] = {.lex_state = 14},
  [12] = {.lex_state = 14},
  [13] = {.lex_state = 14},
  [14] = {.lex_state = 14},
  [15] = {.lex_state = 14},
  [16] = {.lex_state = 14},
  [17] = {.lex_state = 14},
  [18] = {.lex_state = 14},
  [19] = {.lex_state = 14},
  [20] = {.lex_state = 14},
  [21] = {.lex_state = 14},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 14},
  [26] = {.lex_state = 14},
  [27] = {.lex_state = 8},
  [28] = {.lex_state = 8},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_package] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_use] = ACTIONS(1),
    [anon_sym_as] = ACTIONS(1),
    [sym_semver] = ACTIONS(1),
    [sym_block_comment] = ACTIONS(3),
    [sym_slash_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(29),
    [sym_package_decl] = STATE(3),
    [sym_top_level_item] = STATE(4),
    [sym_top_level_use_item] = STATE(15),
    [aux_sym_source_file_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_package] = ACTIONS(7),
    [anon_sym_use] = ACTIONS(9),
    [sym_block_comment] = ACTIONS(3),
    [sym_slash_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(11), 1,
      ts_builtin_sym_end,
    ACTIONS(13), 1,
      anon_sym_use,
    STATE(15), 1,
      sym_top_level_use_item,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    STATE(2), 2,
      sym_top_level_item,
      aux_sym_source_file_repeat1,
  [18] = 5,
    ACTIONS(9), 1,
      anon_sym_use,
    ACTIONS(16), 1,
      ts_builtin_sym_end,
    STATE(15), 1,
      sym_top_level_use_item,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    STATE(6), 2,
      sym_top_level_item,
      aux_sym_source_file_repeat1,
  [36] = 5,
    ACTIONS(9), 1,
      anon_sym_use,
    ACTIONS(16), 1,
      ts_builtin_sym_end,
    STATE(15), 1,
      sym_top_level_use_item,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    STATE(2), 2,
      sym_top_level_item,
      aux_sym_source_file_repeat1,
  [54] = 4,
    ACTIONS(20), 1,
      anon_sym_SLASH,
    STATE(5), 1,
      aux_sym_package_path_repeat1,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(18), 3,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_as,
  [70] = 5,
    ACTIONS(9), 1,
      anon_sym_use,
    ACTIONS(23), 1,
      ts_builtin_sym_end,
    STATE(15), 1,
      sym_top_level_use_item,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    STATE(2), 2,
      sym_top_level_item,
      aux_sym_source_file_repeat1,
  [88] = 4,
    ACTIONS(27), 1,
      anon_sym_SLASH,
    STATE(5), 1,
      aux_sym_package_path_repeat1,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(25), 3,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_as,
  [104] = 4,
    ACTIONS(27), 1,
      anon_sym_SLASH,
    STATE(7), 1,
      aux_sym_package_path_repeat1,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(29), 3,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_as,
  [120] = 3,
    ACTIONS(31), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(18), 3,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_as,
  [133] = 4,
    ACTIONS(33), 1,
      sym_identifier,
    STATE(17), 1,
      sym_use_path,
    STATE(30), 1,
      sym_package_name,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [147] = 4,
    ACTIONS(35), 1,
      sym_identifier,
    STATE(31), 1,
      sym_package_name,
    STATE(33), 1,
      sym_fully_qualified_package_name,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [161] = 3,
    ACTIONS(39), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(37), 2,
      anon_sym_SEMI,
      anon_sym_as,
  [173] = 3,
    ACTIONS(43), 1,
      anon_sym_AT,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(41), 2,
      anon_sym_SEMI,
      anon_sym_as,
  [185] = 2,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(45), 2,
      anon_sym_SEMI,
      anon_sym_as,
  [194] = 2,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(47), 2,
      ts_builtin_sym_end,
      anon_sym_use,
  [203] = 2,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(49), 2,
      ts_builtin_sym_end,
      anon_sym_use,
  [212] = 3,
    ACTIONS(51), 1,
      anon_sym_SEMI,
    ACTIONS(53), 1,
      anon_sym_as,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [223] = 2,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(55), 2,
      ts_builtin_sym_end,
      anon_sym_use,
  [232] = 3,
    ACTIONS(57), 1,
      sym_identifier,
    STATE(22), 1,
      sym_package_path,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [243] = 3,
    ACTIONS(57), 1,
      sym_identifier,
    STATE(13), 1,
      sym_package_path,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [254] = 2,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
    ACTIONS(59), 2,
      ts_builtin_sym_end,
      anon_sym_use,
  [263] = 3,
    ACTIONS(61), 1,
      anon_sym_SEMI,
    ACTIONS(63), 1,
      anon_sym_AT,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [274] = 2,
    ACTIONS(39), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [282] = 2,
    ACTIONS(65), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [290] = 2,
    ACTIONS(67), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [298] = 2,
    ACTIONS(69), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [306] = 2,
    ACTIONS(71), 1,
      sym_semver,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [314] = 2,
    ACTIONS(73), 1,
      sym_semver,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [322] = 2,
    ACTIONS(75), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [330] = 2,
    ACTIONS(77), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [338] = 2,
    ACTIONS(79), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [346] = 2,
    ACTIONS(81), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
  [354] = 2,
    ACTIONS(83), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_block_comment,
      sym_slash_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 18,
  [SMALL_STATE(4)] = 36,
  [SMALL_STATE(5)] = 54,
  [SMALL_STATE(6)] = 70,
  [SMALL_STATE(7)] = 88,
  [SMALL_STATE(8)] = 104,
  [SMALL_STATE(9)] = 120,
  [SMALL_STATE(10)] = 133,
  [SMALL_STATE(11)] = 147,
  [SMALL_STATE(12)] = 161,
  [SMALL_STATE(13)] = 173,
  [SMALL_STATE(14)] = 185,
  [SMALL_STATE(15)] = 194,
  [SMALL_STATE(16)] = 203,
  [SMALL_STATE(17)] = 212,
  [SMALL_STATE(18)] = 223,
  [SMALL_STATE(19)] = 232,
  [SMALL_STATE(20)] = 243,
  [SMALL_STATE(21)] = 254,
  [SMALL_STATE(22)] = 263,
  [SMALL_STATE(23)] = 274,
  [SMALL_STATE(24)] = 282,
  [SMALL_STATE(25)] = 290,
  [SMALL_STATE(26)] = 298,
  [SMALL_STATE(27)] = 306,
  [SMALL_STATE(28)] = 314,
  [SMALL_STATE(29)] = 322,
  [SMALL_STATE(30)] = 330,
  [SMALL_STATE(31)] = 338,
  [SMALL_STATE(32)] = 346,
  [SMALL_STATE(33)] = 354,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [13] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(10),
  [16] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [18] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_package_path_repeat1, 2),
  [20] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_package_path_repeat1, 2), SHIFT_REPEAT(25),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_path, 2),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_path, 1),
  [31] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_package_path_repeat1, 2),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_path, 1),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_name, 1),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_path, 3, .production_id = 1),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_use_path, 5, .production_id = 3),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_top_level_item, 1),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_top_level_use_item, 5, .production_id = 2),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_decl, 3),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_top_level_use_item, 3),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fully_qualified_package_name, 3, .production_id = 1),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [75] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fully_qualified_package_name, 5, .production_id = 3),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_wit() {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_identifier,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
