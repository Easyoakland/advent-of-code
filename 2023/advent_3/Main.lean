import Advent3
import Lean.Data.HashMap
open Classical

@[simp] theorem Nat.sub_add_sub_cancel_middle {n m p : Nat} (h1: m ≤ n) (h2: p ≤ m): (n-m)+(m-p) = n - p := by
  rw [← Nat.add_sub_assoc h2, Nat.sub_add_cancel h1]

theorem Nat.add_mul_lt_mul_of_lt {a b c d : Nat} (zero_lt_c: 0 < c) (zero_lt_d: 0 < d) (a_lt_c: a < c) (b_lt_d: b < d): a+b*c<c*d := by
  calc a + b * c
    _ ≤ a + (d - 1) * c := by
      apply Nat.add_le_add (Nat.le_refl _)
      apply Nat.mul_le_mul (Nat.le_sub_one_of_lt b_lt_d) (Nat.le_refl _)
    _ = a + (d * c - c) := by rw [Nat.sub_one_mul]
    _ ≤ (c - 1) + (d * c - c) := by
      apply Nat.add_le_add
      apply Nat.le_sub_one_of_lt a_lt_c
      exact Nat.le_refl _
    _ = (d * c - c) + (c - 1) := Nat.add_comm ..
    _ = (d * c) - 1 := by
      rw [Nat.sub_add_sub_cancel_middle]
      . simp [Nat.mul_comm, Nat.le_mul_of_pos_right, *]
      . apply Nat.add_one_le_iff.mpr
        simp [*]
    _ < c * d := by
      rw [Nat.mul_comm]
      have c_ne_zero := Nat.ne_of_gt zero_lt_c
      have d_ne_zero := Nat.ne_of_gt zero_lt_d
      have : c * d ≠ 0 := Nat.mul_ne_zero c_ne_zero d_ne_zero
      exact Nat.sub_one_lt this

/- Collect `ForIn'` instance to Array of values and proof of `Membership` -/
def ForIn'.collect [d : Membership α ρ] [ForIn' Id ρ α d] (col:ρ): Array {a//(a:α)∈col} := Id.run do
  let mut acc := #[]
  for h:a in col do
    acc := acc.push $ Subtype.mk a h
  acc

/-- Define a `ToString` instance using pre-existing `Repr` instance -/
def ToString_from_Repr [Repr α]: ToString α :=
  {toString := fun self => ToString.toString $ Repr.reprPrec self 0 : ToString $ α}

structure Coord where
  x: Nat
  y: Nat
deriving Repr, Hashable, BEq

instance: ToString Coord := ToString_from_Repr

instance: Coe (Nat × Nat) Coord where
  coe self := ⟨self.1, self.2⟩
instance: Coe Coord (Nat × Nat) where
  coe self := (self.x, self.y)

instance: Inhabited Coord where
  default := {
    x := Inhabited.default
    y := Inhabited.default
  }

instance: Add Coord where
  add := fun c1 c2 => {
    x := c1.x + c2.x
    y := c1.y + c2.y
  }

instance: Sub Coord where
  sub := fun c1 c2 => {
    x := c1.x - c2.x
    y := c2.y - c2.y
  }

-- Multidimensional array (multiple dimensions is a TODO, currently 2 dim)
structure NDArray (α) where
  -- Data.
  buf: Array α
  -- Dimensions x, y, ...
  dim: Coord
  -- Proof that dimensions are valid for data.
  hdim: buf.size = dim.x * dim.y
deriving Repr

@[simp] theorem NDArray.buf_size_eq_dim_mul (self: NDArray α) : self.buf.size = self.dim.x * self.dim.y := self.hdim

def NDArray.get? (self: NDArray α) (idx: (Nat × Nat)): Option α :=
  if h1: idx.1 >= self.dim.1 then .none
  else if h2: idx.2 >= self.dim.2 then .none
  else self.buf[idx.1 + idx.2 * self.dim.1]'(by
    simp at *
    have : 0 < self.dim.x := Nat.zero_lt_of_lt h1
    have : 0 < self.dim.y := Nat.zero_lt_of_lt h2
    apply Nat.add_mul_lt_mul_of_lt <;> assumption
  )

def NDArray.get (self: NDArray α) (idx: (Fin self.dim.x × Fin self.dim.y)): α :=
  have : 0 < self.dim.x := Nat.zero_lt_of_lt idx.1.isLt
  have : 0 < self.dim.y := Nat.zero_lt_of_lt idx.2.isLt
  have := idx.1.isLt
  have := idx.2.isLt
  have : ↑idx.1 + ↑idx.2 * self.dim.x < self.buf.size := by
        simp <;> apply Nat.add_mul_lt_mul_of_lt .. <;> assumption
  have not_dimx_le_x : ¬(self.dim.x ≤ idx.1) := by simp [Fin.isLt idx.1]
  have not_dimy_le_y : ¬(self.dim.y ≤ idx.2) := by simp [Fin.isLt idx.2]
  have h1 : self.get? (idx.1, idx.2) = Option.some (self.buf[↑idx.1 + ↑idx.2 * self.dim.x]) := by
    simp [NDArray.get?, not_dimx_le_x, not_dimy_le_y]

  match h : self.get? (idx.1, idx.2) with
  | .none => by
    rw [h] at h1
    contradiction
  | .some x => x

instance: GetElem (NDArray α) (Nat×Nat) (α) (fun self idx => idx.1 < self.dim.x ∧ idx.2 < self.dim.y) where
  getElem self idx valid := self.get (⟨⟨idx.1, valid.1⟩, ⟨idx.2, valid.2⟩⟩)

def manhattan_nbd (center: Coord) (radius: Nat): Array Coord := Id.run do
  let x_range := [center.x-radius:center.x+radius+1]
  let y_range := [center.y-radius:center.y+radius+1]
  let mut out := #[];
  for x in x_range do
    for y in y_range do
      out := out.push {x:=x,y:=y:Coord}
  out

example: manhattan_nbd (1,1) 1 = #[↑(0,0), ↑(0, 1), ↑(0, 2), ↑(1, 0), ↑(1, 1), ↑(1, 2), ↑(2, 0), ↑(2, 1), ↑(2, 2)] := rfl
#reduce manhattan_nbd (2,2) 1

instance: Inhabited $ NDArray α where
  default := {
    buf := Inhabited.default
    dim := Inhabited.default
    hdim := by rfl
  }

#eval {
  buf := #[1,2
          ,3,4]
  dim := Coord.mk 2 2
  hdim := by simp : NDArray _
}.get? (1,1)

def parse_input (path: System.FilePath): IO $ Option (NDArray Char) := do
  let res ← IO.FS.lines path
  let map: Option _ := res.foldlM (init:=(Inhabited.default: NDArray Char)) fun acc line =>
    let line := .mk line.data
    -- Empty case
    if h: acc.dim.x = 0 then pure {
      buf := acc.buf.append line
      dim := Coord.mk line.size 1
      hdim := by simp [acc.hdim, h]
    }
    -- Invalid data
    else if h: line.size != acc.dim.1 then none
    -- New valid line
    else
      pure {
        buf := acc.buf.append line
        dim := Coord.mk acc.dim.x (acc.dim.y+1)
        hdim := by
          simp at h
          simp
          rw [Nat.mul_add]
          simp [acc.hdim, h]
      }
  pure map

structure PartNumberSpan where
  start: (Nat×Nat)
  length: Nat
deriving Repr

instance: ToString PartNumberSpan := ToString_from_Repr

/-- All the coords contained in the span. -/
def PartNumberSpan.coordsArray (self: PartNumberSpan): Array (Nat × Nat) :=
  .range self.length
    |> .map (· + self.start.1, self.start.2)

structure PartNumberSpan.Coords where
  start : Coord
  length : Nat

def PartNumberSpan.coords (self:PartNumberSpan) : Coords where
  start := self.start
  length := self.length

@[simp] theorem PartNumberSpan.coords_start_eq_self_start (self:PartNumberSpan) : self.coords.start = self.start := rfl

@[simp] def PartNumberSpan.Coords.mem (self:PartNumberSpan.Coords) (n:Nat×Nat): Prop := n.1 ∈ [self.start.1:self.start.1 + self.length] ∧ n.2 = self.start.2

instance: Membership Coord PartNumberSpan.Coords where
  mem a b := PartNumberSpan.Coords.mem a b

@[simp] theorem PartNumberSpan.Coords.mem_y_eq_start (self:Coords) (c:Coord) : c ∈ self → c.y = self.start.y := by simp [Membership.mem]

@[simp] theorem PartNumberSpan.Coords.start_mem_of_length_ne_0 (self:PartNumberSpan.Coords) (h:0 ≠ self.length): self.start ∈ self := by
  simp [(. ∈ .), h]
  exact Nat.zero_lt_of_ne_zero (Ne.symm h)

-- Computable instances of Decidable for membership (not `Classical.propDecidable`)
instance: Decidable $ Std.instMembershipNatRange.mem r n := instDecidableAnd
instance: Decidable $ instMembershipCoordCoords.mem cs c := instDecidableAnd

@[simp] theorem Membership.mem.Coords_upper {i : Coord} {r : PartNumberSpan.Coords} (h : i ∈ r) : i.x < r.start.1 + r.length := h.1.upper

@[simp] theorem Membership.mem.Coords_lower {i : Coord} {r : PartNumberSpan.Coords} (h : i ∈ r) : r.start.1 ≤ i.x := h.1.lower

@[simp] theorem PartNumberSpan.Coords.not_mem_length_0 s n : ¬(n ∈ PartNumberSpan.Coords.mk s 0) := by
  simp [instMembershipCoordCoords]
  intro h
  have := Nat.not_lt.mpr h.lower
  exact False.elim $ ‹¬n.x < s.1› h.upper

instance: EmptyCollection PartNumberSpan.Coords := ⟨PartNumberSpan.Coords.mk Inhabited.default 0⟩

@[simp] theorem PartNumberSpan.Coords.empty_eq : ∅ = PartNumberSpan.Coords.mk Inhabited.default 0 := rfl

@[simp] theorem PartNumberSpan.Coords.not_mem_empty n : ¬(n ∈ (∅:PartNumberSpan.Coords)) := fun h =>
  by simp only [empty_eq, Coords.not_mem_length_0] at h

def PartNumberSpan.Coords.forIn'.aux [Monad m] (next:Coord) (self : PartNumberSpan.Coords) (h:next∈self) (b:β) (f: (a : Coord) → a ∈ self → β → m (ForInStep β)) : m β := do
  match (← f next h b) with
  | ForInStep.done b2 => pure b2
  | ForInStep.yield b2 =>
    let next := next + ↑(1,0)
    if h:next∈self then
    PartNumberSpan.Coords.forIn'.aux next self h b2 f
    else
      pure b2
  termination_by ↑self.start+↑(self.length,0) - next
  decreasing_by
    rename_i prev prev_mem_self
    simp [(. + .), (. - .), sizeOf, Coord._sizeOf_1]
    simp [Add.add, Sub.sub]
    have : next.x ≤ self.start.1 + self.length := Nat.le_of_succ_le h.Coords_upper
    have h2: (a b c :Nat) → c < b → b ≤ a → a - b < a - c := by omega
    suffices prev.x < (prev.x + 1) ∧ prev.x ≤ self.start.1 + self.length by
      apply h2
      . exact Nat.lt_add_one ..
      . assumption
    apply And.intro
    . exact Nat.lt_add_one ..
    . exact Nat.le_of_succ_le this
    done

instance: ForIn' m PartNumberSpan.Coords Coord instMembershipCoordCoords where
  forIn' x b f := if h:0=x.length then pure b else PartNumberSpan.Coords.forIn'.aux x.start x (by simp [h]) b f

theorem PartNumberSpan.Coords.eq_coords_array (self:Coords) (p:PartNumberSpan): p.start = self.start → p.length = self.length → ∀ c, c ∈ self → c ∈ (p.coordsArray |>.map (λ x => ↑x)):= fun h h2 c h4 => by
  simp [coordsArray]
  apply Exists.intro (c.x - p.start.1)
  have h5 : p.start.1 = self.start.1 := by simp only [h]
  have h6 : p.start.2 = self.start.2 := by simp only [h]
  have : c.x < self.start.x + self.length := h4.Coords_upper
  have := h4.Coords_lower
  apply And.intro
  . apply Array.mem_toList.mp
    rw [Array.toList_range]
    apply List.mem_range.mpr
    rw [h5, h2]
    suffices c.x < self.length + self.start.x by omega
    rw [Nat.add_comm]
    assumption
  . rw [Nat.sub_add_cancel, h6]
    have : c.y = self.start.y := mem_y_eq_start self c h4
    rw [← this]
    . rw [h5]
      assumption


#eval (PartNumberSpan.mk (3,3) 3).coordsArray
#eval ForIn.forIn (m:=Id) (PartNumberSpan.Coords.mk (3,3) 3) #[] (fun c b =>
  pure $ ForInStep.yield $ b.push c
)
#eval Id.run do
  let mut v := #[]
  for c in PartNumberSpan.Coords.mk (3,3) 3 do
    v := v.push c
  v
#eval Id.run do
  let mut v := #[]
  for h:c in PartNumberSpan.Coords.mk (3,3) 3 do
    have := h
    v := v.push c
  v


def radix_10 (c: Char): Prop :=
  c = '0'
∨ c = '1'
∨ c = '2'
∨ c = '3'
∨ c = '4'
∨ c = '5'
∨ c = '6'
∨ c = '7'
∨ c = '8'
∨ c = '9'

-- Don't use `Classical.propDecidable` for radix_10
instance: Decidable $ radix_10 c := instDecidableOr

example (c:Char) : Bool := decide $ radix_10 c

def parse_Nat' (s: String) (_: c ∈ s.data → radix_10 c) : Nat := Id.run do
  let mut num := 0
  for h:c in s.data do
    let digit ← match c with
    | '0' => 0
    | '1' => 1
    | '2' => 2
    | '3' => 3
    | '4' => 4
    | '5' => 5
    | '6' => 6
    | '7' => 7
    | '8' => 8
    | '9' => 9
    | _ => by assumption
    num := num * 10 + digit
  num

def parse_Nat (s: String): Option Nat := do
  let mut num := 0
  for c in s.data do
    let digit ← match c with
    | '0' => Option.some 0
    | '1' => Option.some 1
    | '2' => Option.some 2
    | '3' => Option.some 3
    | '4' => Option.some 4
    | '5' => Option.some 5
    | '6' => Option.some 6
    | '7' => Option.some 7
    | '8' => Option.some 8
    | '9' => Option.some 9
    | _ => Option.none
    num := num * 10 + digit
  num

-- range appears [inclusive:exclusive]
#eval Id.run do
  let mut acc := []
  for x in [0:3] do
    acc := acc.cons x
  acc.reverse


def Option.unwrap (o:Option α): α :=
  match o with
  | .none => sorry
  | .some x => x

def part1 (parsed: NDArray Char): Option Nat := do
  let mut parts: Array (PartNumberSpan × Nat) := #[]
  for hy:y in [0:parsed.dim.y] do
    have y_lt_dimy : y < parsed.dim.y := hy.upper
    -- commented out below is how `hy.right` was found, though the `hy.upper` theorem is equivalent
    -- by
      -- conv at hy =>
        -- unfold Membership.mem
        -- unfold inferInstance
        -- unfold Std.instMembershipNatRange
        -- simp only
      -- exact
      -- hy.right

    -- Finish a part number and return (span, value).
    let end_part length part_start {h1: part_start.1 + length ≤ parsed.dim.x} {h2:part_start.2 < parsed.dim.y}:=
      if _:length > 0 then do
        let part : PartNumberSpan := {start := part_start, length := length  }
        let val ← ForIn'.collect part.coords
          |>.map (fun ⟨loc, h⟩ => (parsed[(loc:(Nat×Nat))]'(by
            simp only
            apply And.intro
            .  have := part.coords_start_eq_self_start ▸ h.Coords_upper
               simp only at this
               calc loc.x
                  < part_start.fst + part.coords.length := this
                _ ≤ parsed.dim.x := h1
            . have : loc.y = part.start.2 := PartNumberSpan.Coords.mem_y_eq_start part.coords loc $ h
              rw [‹loc.y=_›]
              assumption
          )))
          |>.foldl (init := "") (fun s c =>
                s.push $ c
              )
          |> parse_Nat
        pure (part, val)
      else Option.none

    let mut part_start: {a:Nat//a≤parsed.dim.x}×{a:Nat//a=y}:=
      (⟨0, Nat.zero_le parsed.dim.x⟩, ⟨y,rfl⟩)
    for hx:x in [0:parsed.dim.x] do
      have x_lt_dimx : x < parsed.dim.x := hx.upper

      let c := parsed.get (Fin.mk x x_lt_dimx, Fin.mk y y_lt_dimy)
      let n := parse_Nat $ String.mk [c]

      if n.isNone then
        let length := x-part_start.1
        match end_part length (part_start |> λx=>(x.1,x.2)) (h1:=by
          have := part_start.1.property
          simp only
          omega
        ) (h2:= by
          have := part_start.2.property
          simp only [this]
          exact y_lt_dimy
        ) with
        | .none => ()
        | .some part => parts := parts.push part
        part_start := (⟨x+1,x_lt_dimx⟩,⟨y,rfl⟩)
    -- don't forget to end parts which end at a newline
    let length := parsed.dim.x-part_start.1
    match end_part length (part_start |> λx=>(x.1,x.2)) (h1:=by
      have := part_start.1.property
      simp only
      omega
    ) (h2:= by
          have := part_start.2.property
          simp only [this]
          exact y_lt_dimy
        ) with
    | .none => ()
    | .some part => parts := parts.push part

  -- Active parts are those whose manhattan neighborhood contains a symbol.
  -- Inefficiently implemented by checking the manhattan nbd of each digit which is equivalent.
  let active_parts := parts.map (fun (span, _val) =>
    span.coordsArray.any (fun coord =>
      (manhattan_nbd coord 1).any (fun coord =>
        let c := (parsed.get? coord).map (fun c => c != '.' && !c.isDigit)
        match c with
        | .some true => true
        | _ => false
      )
    )
  )

  -- The result is the sum of active parts' numbers
  let res := (parts.zip active_parts).foldl (init := 0) (fun acc ((_span, val), active) =>
    if active then acc + val
    else acc
  )

  pure res

def part2 (parsed: NDArray Char): Option Nat := do
  let mut parts: Array (PartNumberSpan × Nat) := #[]
  for hy:y in [0:parsed.dim.y] do
    have y_lt_dimy : y < parsed.dim.y := hy.upper

    -- Finish a part number and return (span, value).
    let end_part length part_start :=
      if length > 0 then
        let part : PartNumberSpan := {start := part_start, length := length}
        let val := part.coordsArray
          |>.map (fun loc => parsed[loc]?.unwrap)
          |>.foldl (init := "") (fun s c =>
                s.push $ c
              )
          |> parse_Nat
          |> Option.unwrap
        pure (part, val)
      else Option.none

    let mut part_start := (0, y)
    for hx:x in [0:parsed.dim.x] do
      have x_lt_dimx : x < parsed.dim.x := hx.upper

      let c := parsed.get (Fin.mk x x_lt_dimx, Fin.mk y y_lt_dimy)
      let n := parse_Nat $ String.mk [c]

      if n.isNone then
        let length := x-part_start.1
        match end_part length part_start with
        | .none => ()
        | .some part => parts := parts.push part
        part_start := (x+1,y)
    -- don't forget to end parts which end at a newline
    let length := parsed.dim.x-part_start.1
    match end_part length part_start with
    | .none => ()
    | .some part => parts := parts.push part

  -- Find adjacent gears for each part.
  let gears_per_part := parts.map (fun (span, number) =>
    let nbd := span.coordsArray.map (fun coord: (Nat × Nat) => manhattan_nbd coord 1) |>.flatten
    let gears_in_nbd: Array Coord := nbd.foldl (init:=#[]) (fun acc (coord:Coord) =>
      -- dedup while creating because information about number of digits adjacent to gears it not used.
      if (parsed.get? coord) == Option.some '*' && (!acc.contains coord) then acc.push coord
      else acc
    )
    (number, gears_in_nbd)
  )

  -- coord_of_gear -> part_list_in_nbd
  let parts_per_gear: Std.HashMap Coord (Array Nat) := gears_per_part.foldl (init:=Std.HashMap.empty) (fun acc (part_num, gears_in_nbd) =>
    gears_in_nbd.foldl (init:=acc) (fun acc gear =>
      -- Add the part number to the entry in the map for the gear
      match acc.get? gear with
      | .none => acc.insert gear #[part_num]
      | .some adj_parts => acc.insert gear $ adj_parts.push part_num
    )
  )

  -- Active gears are those with exactly 2 part number neighbors
  let active_gears := parts_per_gear.toArray |>.filter (fun (_gear_coord, part_nums) => part_nums.size = 2) |>.map (fun (_gear_coord, part_nums) => part_nums)

  -- The result is the sum of all gear ratio's
  let res := active_gears.foldl (init:=0) (fun acc parts =>
    let gear_ratio := parts.foldl (init:=1) (. * .)
    acc + gear_ratio
  )

  pure res

def try_main : OptionT IO Unit := do
  let parsed ← parse_input "inputtest.txt"
  let ans := part1 parsed
  IO.println s!"part 1 test: {ans}"
  assert! 4361 = ans.unwrap

  let parsed ← parse_input "inputtest2.txt"
  let ans := part1 parsed
  IO.println s!"part 1 test2: {ans}"

  let parsed ← parse_input "input.txt"
  let ans := part1 parsed
  assert! 537066 < ans.unwrap
  IO.println s!"part 1: {ans}"
  assert! 538046 = ans.unwrap

  let parsed ← parse_input "inputtest.txt"
  let ans := part2 parsed
  IO.println s!"part 2 test: {ans}"
  assert! 467835 = ans.unwrap

  let parsed ← parse_input "input.txt"
  let ans := part2 parsed
  IO.println s!"part 2: {ans}"
  assert! 81709807 = ans.unwrap

def main : IO Unit := do
  let res ← try_main.run
  match res with
  | .none => IO.println "error"
  | some _ => IO.println "done"
