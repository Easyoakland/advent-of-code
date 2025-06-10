import Lake
open Lake DSL

package "advent_3" where
  -- add package configuration options here

lean_lib «Advent3» where
  -- add library configuration options here

@[default_target]
lean_exe "advent_3" where
  root := `Main
