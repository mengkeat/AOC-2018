open System.IO

let dat = File.ReadAllLines ( __SOURCE_DIRECTORY__ + "/day18.txt" )

let neigh x y = [(x-1,y-1); (x,y-1); (x+1, y-1); (x-1,y); (x+1,y); (x-1,y+1); (x,y+1); (x+1,y+1)] 
                |> List.filter (fun (r,c) -> (r>=0)&&(r<50)&&(c>=0)&&(c<50))

type NeighCounts = { Empty:int; Trees:int; Lumber:int; }

let addNeighbours acc e = 
    match e with 
    | '.' -> {acc with Empty = acc.Empty+1 }
    | '|' -> {acc with Trees = acc.Trees+1 }
    | _ -> {acc with Lumber = acc.Lumber+1 } 

let GetNeighCounts x y (arr: string []) = 
    neigh x y  
    |> List.map (fun (x,y) -> arr.[x].[y])
    |> List.fold addNeighbours {Empty=0; Trees=0; Lumber=0}

let nextElem elem ncount = 
    match elem with
    | '.' -> if ncount.Trees>=3 then '|' else '.'
    | '|' -> if ncount.Lumber>=3 then '#' else '|'
    | _ -> if ncount.Lumber>=1 && ncount.Trees>=1 then '#' else '.'

let nextArray (curr: string[]) =  
    [for x in 0..49 do
        [for y in 0..49 ->
            nextElem curr.[x].[y] (GetNeighCounts x y curr)] |> Array.ofList |> System.String ]  |> Array.ofList

let sumArray arr = Seq.fold addNeighbours {Empty=0; Trees=0; Lumber=0} arr
let resourceValue (curr: string[]) = curr |> Seq.reduce (+) |> sumArray |> (fun x-> x.Trees * x.Lumber)

let part1 = List.fold (fun acc e -> nextArray acc) dat [1..10]
            |> resourceValue

let rec findCycle grid (sofar: List<string>) n = 
    let checkexist e = List.contains e sofar
    let currRes = grid |> Seq.reduce (+)
    if checkexist currRes then (n, sofar@[currRes], currRes)
    else findCycle (nextArray grid) (sofar@[currRes]) (n+1)

let l, lst, num = findCycle dat [] 0
let [p1;p2] = List.indexed lst |> List.filter (fun (i,e) -> e=num) 
let cycleLength, offset = fst p2 - fst p1, fst p1
let part2 =  lst.[offset + (1000000000-offset)%cycleLength ] 
            |> Seq.chunkBySize 50 
            |> Seq.map System.String
            |> Array.ofSeq
            |> resourceValue
