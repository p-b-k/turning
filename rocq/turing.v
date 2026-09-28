(* ****************************************************************************************************************** *
 * Attempt to formalize a Turing Machine in Rocq.
 * Or at least attempt to refresh my Rocq abilities.
 * ****************************************************************************************************************** *)

(* The sequence of cells *)
Parameter U : Set.

(* Equality Relationships *)

(* Next and Previous functions *)
Parameter next : U -> U.
Parameter next_inf_pos : forall u : U, exists v : U, v = (next u).
Parameter next_inf_neg : forall u : U, exists v : U, u = (next v).
Parameter next_uniq : forall u v : U, (next u) = (next v) -> u = v.

Parameter prev : U -> U.
Parameter prev_def : forall u v, ((next u) = v) <-> ((prev v) = u).

(* Properties of next and previous *)
Theorem next_inv_prev : forall u : U, (next (prev u)) = u.
Proof.
 intros.
 apply prev_def.
 reflexivity.
Qed.

Theorem prev_inv_next : forall u : U, (prev (next u)) = u.
Proof.
 intros.
 apply prev_def.
 reflexivity.
Qed.

Definition is_next (u v : U) := (next u) = v.
Definition is_prev (u v : U) := (prev u) = v.

Theorem is_prev_def : forall u v : U, is_next u v -> is_prev v u.
Proof.
 intros u v.
 unfold is_prev.
 unfold is_next.
 intro H.
 rewrite <- H.
 apply prev_inv_next.
Qed.

Theorem next_excl_r : forall u v w, is_next u v /\ is_next u w -> v = w.
Proof.
 intros u v w.
 unfold is_next.
 unfold is_prev.
 intro H.
 destruct H as [H1 H2].
 rewrite <- H1.
 rewrite <- H2.
 reflexivity.
Qed.

Theorem next_excl_l : forall u v w, is_next v u /\ is_next w u -> v = w.
Proof.
 intros u v w.
 unfold is_next.
 intro H.
 destruct H as [H1 H2].
 apply next_uniq.
 rewrite H1.
 rewrite H2.
 reflexivity.
Qed.

  
(* Star and Plus predicates (as in repeated application of a is_prev/is_next) *)

(* Define is_n_star inductively:
   If is_next x y then x is before y
   If if the next number after x is before y then x is also before y *)
Inductive is_n_star : U -> U -> Prop :=
 | P_base : forall x y : U, x = y -> is_n_star x y
 | P_step : forall x y : U, is_n_star (next x) y -> is_n_star x y.

(* Define related terms in terms of is_n_star *)
Definition is_p_star (u v : U) := is_n_star v u.
Definition is_n_plus (u v : U) := is_n_star u v /\ ~(is_next u v).
Definition is_p_plus (u v : U) := is_n_plus v u.

(* SKIP AFTER THIS POINT     **************************************************************************************** * 

(* Properties of n_plus and p_plus *)
Parameter n_plus_trans : forall u v w : U, is_n_plus u v /\ is_n_plus v w -> is_n_plus u w.
Parameter n_plus_inv : forall u v : U, is_n_plus u v -> is_p_plus v u.
Parameter n_plus_comp : forall u v : U, is_n_plus u v /\ is_n_plus v u /\ u = v.
Parameter n_plus_excl : forall u v : U, is_n_plus u v -> ~ is_n_plus v u.

Parameter p_plus_trans : forall u v w : U, is_p_plus u v /\ is_p_plus v w -> is_p_plus u w.
Parameter p_plus_inv : forall u v : U, is_p_plus u v -> is_n_plus v u.
Parameter p_plus_comp : forall u v : U, is_p_plus u v /\ is_p_plus v u /\ u = v.
Parameter p_plus_excl : forall u v : U, is_p_plus u v -> ~ is_p_plus v u.

(*
 * Create function from U to a universe
 *)

(* The alphabet of possible cell values *)
Parameter A : Set.


 * END OF SKIPPED BLOCK      **************************************************************************************** *)
