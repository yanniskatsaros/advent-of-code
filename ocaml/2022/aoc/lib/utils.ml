let read_file file =
  In_channel.with_open_bin file In_channel.input_all


let sum =
  List.fold_left ( + ) 0
(** [sum] returns the sum of the items in the given [int list] *)


(* unfortunately the stdlib List doesn't have take or drop *)
let take k lst =
  (* tail recursive solution *)
  let rec go k lst acc = match k with
    | 0 -> Some acc
    | _ -> begin match lst with
      | [] -> None
      | h :: t -> go (k-1) t (h::acc)
    end
  in
  go k lst []
(** [take] returns the first [k] items from the [lst] *)

let chunk i seq =
  let first = List.of_seq (Seq.take i seq) in
  let rest = Seq.drop i seq in
  first, rest
(** [chunk] returns the first [i] items from the [seq] along with the remainder of the sequence *)

let chunks i seq =
  let rec go i seq acc = match chunk i seq with
    | [], _ -> acc
    | xs, rest -> go i rest (xs::acc)
  in
  (* reversal is needed due to the recursive expansion *)
  List.rev (go i seq [])
(** [chunks] exhausts the sequence returning a list with the items grouped
    in individual lists each of size [i] *)
