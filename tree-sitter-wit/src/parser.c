#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 25
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 19
#define ALIAS_COUNT 0
#define TOKEN_COUNT 10
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 3
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 3

enum ts_symbol_identifiers {
  sym_identifier = 1,
  anon_sym_package = 2,
  anon_sym_SEMI = 3,
  anon_sym_COLON = 4,
  anon_sym_AT = 5,
  anon_sym_SLASH = 6,
  sym_semver = 7,
  sym__block_comment = 8,
  sym__slash_comment = 9,
  sym_source_file = 10,
  sym_package_decl = 11,
  sym_fully_qualified_package_name = 12,
  sym_package_name = 13,
  sym_package_path = 14,
  sym_top_level_item = 15,
  aux_sym_source_file_repeat1 = 16,
  aux_sym_package_name_repeat1 = 17,
  aux_sym_package_path_repeat1 = 18,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_package] = "package",
  [anon_sym_SEMI] = ";",
  [anon_sym_COLON] = ":",
  [anon_sym_AT] = "@",
  [anon_sym_SLASH] = "/",
  [sym_semver] = "semver",
  [sym__block_comment] = "_block_comment",
  [sym__slash_comment] = "_slash_comment",
  [sym_source_file] = "source_file",
  [sym_package_decl] = "package_decl",
  [sym_fully_qualified_package_name] = "fully_qualified_package_name",
  [sym_package_name] = "package_name",
  [sym_package_path] = "package_path",
  [sym_top_level_item] = "top_level_item",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_package_name_repeat1] = "package_name_repeat1",
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
  [sym_semver] = sym_semver,
  [sym__block_comment] = sym__block_comment,
  [sym__slash_comment] = sym__slash_comment,
  [sym_source_file] = sym_source_file,
  [sym_package_decl] = sym_package_decl,
  [sym_fully_qualified_package_name] = sym_fully_qualified_package_name,
  [sym_package_name] = sym_package_name,
  [sym_package_path] = sym_package_path,
  [sym_top_level_item] = sym_top_level_item,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_package_name_repeat1] = aux_sym_package_name_repeat1,
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
  [sym_semver] = {
    .visible = true,
    .named = true,
  },
  [sym__block_comment] = {
    .visible = false,
    .named = true,
  },
  [sym__slash_comment] = {
    .visible = false,
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
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_package_name_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_package_path_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_package = 1,
  field_path = 2,
  field_version = 3,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_package] = "package",
  [field_path] = "path",
  [field_version] = "version",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_package, 0},
    {field_path, 2},
  [2] =
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(16);
      if (lookahead == '/') ADVANCE(20);
      if (lookahead == '0') ADVANCE(28);
      if (lookahead == ':') ADVANCE(18);
      if (lookahead == ';') ADVANCE(17);
      if (lookahead == '@') ADVANCE(19);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(14)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(27);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(29);
      END_STATE();
    case 1:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead == '/') ADVANCE(31);
      END_STATE();
    case 2:
      if (lookahead == '*') ADVANCE(2);
      if (lookahead == '/') ADVANCE(30);
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
      if (lookahead == '0') ADVANCE(21);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(22);
      END_STATE();
    case 11:
      if (lookahead == '0') ADVANCE(23);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(24);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(25);
      END_STATE();
    case 12:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(12);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(25);
      END_STATE();
    case 13:
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 14:
      if (eof) ADVANCE(16);
      if (lookahead == '/') ADVANCE(1);
      if (lookahead == '0') ADVANCE(28);
      if (lookahead == ';') ADVANCE(17);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(14)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(27);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(29);
      END_STATE();
    case 15:
      if (eof) ADVANCE(16);
      if (lookahead == '/') ADVANCE(1);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(15)
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(29);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(3);
      if (lookahead == '/') ADVANCE(31);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '+') ADVANCE(13);
      if (lookahead == '-') ADVANCE(11);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '+') ADVANCE(13);
      if (lookahead == '-') ADVANCE(11);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(22);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '+') ADVANCE(13);
      if (lookahead == '.') ADVANCE(11);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(12);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(25);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '+') ADVANCE(13);
      if (lookahead == '.') ADVANCE(11);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(24);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(25);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '+') ADVANCE(13);
      if (lookahead == '.') ADVANCE(11);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(25);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_semver);
      if (lookahead == '.') ADVANCE(13);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(26);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '.') ADVANCE(9);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(29);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '.') ADVANCE(9);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(29);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(29);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym__block_comment);
      if (lookahead == '*') ADVANCE(2);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym__slash_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(31);
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
      if (lookahead == 'p') ADVANCE(1);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == 'a') ADVANCE(2);
      END_STATE();
    case 2:
      if (lookahead == 'c') ADVANCE(3);
      END_STATE();
    case 3:
      if (lookahead == 'k') ADVANCE(4);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(5);
      END_STATE();
    case 5:
      if (lookahead == 'g') ADVANCE(6);
      END_STATE();
    case 6:
      if (lookahead == 'e') ADVANCE(7);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_package);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 15},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 15},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 15},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 15},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 15},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 8},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
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
    [sym_semver] = ACTIONS(1),
    [sym__block_comment] = ACTIONS(3),
    [sym__slash_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(19),
    [sym_package_decl] = STATE(6),
    [sym_top_level_item] = STATE(7),
    [aux_sym_source_file_repeat1] = STATE(7),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_package] = ACTIONS(7),
    [sym__block_comment] = ACTIONS(3),
    [sym__slash_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(11), 1,
      anon_sym_SLASH,
    STATE(2), 1,
      aux_sym_package_path_repeat1,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
    ACTIONS(9), 2,
      anon_sym_SEMI,
      anon_sym_AT,
  [15] = 4,
    ACTIONS(16), 1,
      anon_sym_SLASH,
    STATE(2), 1,
      aux_sym_package_path_repeat1,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
    ACTIONS(14), 2,
      anon_sym_SEMI,
      anon_sym_AT,
  [30] = 4,
    ACTIONS(16), 1,
      anon_sym_SLASH,
    STATE(3), 1,
      aux_sym_package_path_repeat1,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
    ACTIONS(18), 2,
      anon_sym_SEMI,
      anon_sym_AT,
  [45] = 4,
    ACTIONS(20), 1,
      sym_identifier,
    STATE(21), 1,
      sym_package_name,
    STATE(23), 1,
      sym_fully_qualified_package_name,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [59] = 3,
    ACTIONS(22), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
    STATE(9), 2,
      sym_top_level_item,
      aux_sym_source_file_repeat1,
  [71] = 3,
    ACTIONS(22), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
    STATE(10), 2,
      sym_top_level_item,
      aux_sym_source_file_repeat1,
  [83] = 3,
    ACTIONS(24), 1,
      anon_sym_SLASH,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
    ACTIONS(9), 2,
      anon_sym_SEMI,
      anon_sym_AT,
  [95] = 3,
    ACTIONS(26), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
    STATE(10), 2,
      sym_top_level_item,
      aux_sym_source_file_repeat1,
  [107] = 3,
    ACTIONS(28), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
    STATE(10), 2,
      sym_top_level_item,
      aux_sym_source_file_repeat1,
  [119] = 3,
    ACTIONS(30), 1,
      anon_sym_COLON,
    STATE(13), 1,
      aux_sym_package_name_repeat1,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [130] = 3,
    ACTIONS(32), 1,
      sym_identifier,
    STATE(14), 1,
      sym_package_path,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [141] = 3,
    ACTIONS(34), 1,
      anon_sym_COLON,
    STATE(13), 1,
      aux_sym_package_name_repeat1,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [152] = 3,
    ACTIONS(37), 1,
      anon_sym_SEMI,
    ACTIONS(39), 1,
      anon_sym_AT,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [163] = 3,
    ACTIONS(41), 1,
      anon_sym_COLON,
    STATE(11), 1,
      aux_sym_package_name_repeat1,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [174] = 2,
    ACTIONS(43), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [182] = 2,
    ACTIONS(45), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [190] = 2,
    ACTIONS(47), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [198] = 2,
    ACTIONS(49), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [206] = 2,
    ACTIONS(51), 1,
      sym_identifier,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [214] = 2,
    ACTIONS(53), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [222] = 2,
    ACTIONS(55), 1,
      sym_semver,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [230] = 2,
    ACTIONS(57), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
  [238] = 2,
    ACTIONS(59), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym__block_comment,
      sym__slash_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 15,
  [SMALL_STATE(4)] = 30,
  [SMALL_STATE(5)] = 45,
  [SMALL_STATE(6)] = 59,
  [SMALL_STATE(7)] = 71,
  [SMALL_STATE(8)] = 83,
  [SMALL_STATE(9)] = 95,
  [SMALL_STATE(10)] = 107,
  [SMALL_STATE(11)] = 119,
  [SMALL_STATE(12)] = 130,
  [SMALL_STATE(13)] = 141,
  [SMALL_STATE(14)] = 152,
  [SMALL_STATE(15)] = 163,
  [SMALL_STATE(16)] = 174,
  [SMALL_STATE(17)] = 182,
  [SMALL_STATE(18)] = 190,
  [SMALL_STATE(19)] = 198,
  [SMALL_STATE(20)] = 206,
  [SMALL_STATE(21)] = 214,
  [SMALL_STATE(22)] = 222,
  [SMALL_STATE(23)] = 230,
  [SMALL_STATE(24)] = 238,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_package_path_repeat1, 2),
  [11] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_package_path_repeat1, 2), SHIFT_REPEAT(20),
  [14] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_path, 2),
  [16] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [18] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_path, 1),
  [20] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [22] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [24] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_package_path_repeat1, 2),
  [26] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2),
  [28] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [30] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_name, 2),
  [32] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [34] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_package_name_repeat1, 2), SHIFT_REPEAT(16),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fully_qualified_package_name, 3, .production_id = 1),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_name, 1),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_package_decl, 3),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_package_name_repeat1, 2),
  [49] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fully_qualified_package_name, 5, .production_id = 2),
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
