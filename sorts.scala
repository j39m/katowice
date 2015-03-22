import scala.util.Random
import scala.collection.mutable.MutableList

object ManySorts { 

  // main function, where any sort can be deployed. 
  def main (args: Array[String]) { 
    val klaus : Vector[Int] = random_int_vector(26, int_lim=104); 
    println("Initially we have: \n" + klaus); 
    val tama : Vector[Int] = MergeSort.mergesort(klaus); 
    println("Then we mergesort to get: \n" + tama); 
    val qtama : Vector[Int] = QuickSort.quicksort(klaus); 
    println("Then we quicksort to get: \n" + qtama); 
    val btama : Vector[Int] = BubbleSort.bubble_sort(klaus); 
    println("Also, bubble sort comes out: \n" + btama); 
    return; 
  } 

  // generator for random lists of numbers 
  def random_int_vector (length_lim: Int = 52, int_lim: Int = Int.MaxValue): Vector[Int] = { 
    var vector : Vector[Int] = Vector(); 
    var counter : Int = 0; 
    while (counter < length_lim) { 
      vector ++= Vector(Random.nextInt(int_lim)); 
      counter += 1; 
    } 
    return vector; 
  } 

} 


object BubbleSort { 

  // vector to MutableList converter
  def v_to_ml (convert: Vector[Int]): MutableList[Int] = { 
    var retval = new MutableList[Int](); 
    for (i : Int <- convert) { 
      retval += i; 
    } 
    return retval; 
  } 

  def bubble_sort (sortee: Vector[Int]): Vector[Int] = { 
    if (sortee.length <= 1) { 
      return sortee; 
    } 
    var local_list : MutableList[Int] = v_to_ml(sortee); 
    var had_to_swap = true; 
    while (had_to_swap) { 
      had_to_swap = false; 
      var i = 0; 
      var j = 1; 
      while(j<local_list.length) { 
        if (local_list(i) > local_list(j)) { 
          var tmp = local_list(i); 
          local_list(i) = local_list(j); 
          local_list(j) = tmp; 
          had_to_swap = true; 
        } 
        i += 1; 
        j += 1; 
      } 
    } 
    return local_list.toVector; 
  } 

} 


object MergeSort { 

  def mergesort_merge (listI: Vector[Int], listII: Vector[Int]): Vector[Int] = { 

    var merged : Vector[Int] = Vector(); 
    var i : Int = 0; 
    var j : Int = 0; 

    while ((i<listI.length)&&(j<listII.length)) { 
      if (listI(i) < listII(j)) { 
        merged ++= Vector(listI(i)); 
        i += 1; 
      } else { 
        merged ++= Vector(listII(j)); 
        j += 1; 
      } 
    } 

    // at this point, one or more lists have emptied. 
    // stuff the remainder of the list directly into the return. 
    while (i<listI.length) { 
      merged ++= Vector(listI(i)); 
      i += 1; 
    } 
    while (j<listII.length) { 
      merged ++= Vector(listII(j)); 
      j += 1; 
    } 

    return merged; 
  } 

  def mergesort (sortee: Vector[Int]): Vector[Int] = { 

    if (sortee.length <= 1) { 
      return sortee; 
    } 
    
    val pivot : Int = sortee.length / 2; 
    var i : Int = 0; 
    var listI : Vector[Int] = Vector(); 
    var listII : Vector[Int] = Vector(); 
    var sorted_listI : Vector[Int] = Vector(); 
    var sorted_listII : Vector[Int] = Vector(); 

    while (i < pivot) { 
      listI ++= Vector(sortee(i)); 
      i += 1; 
    } 
    while (i < sortee.length) { 
      listII ++= Vector(sortee(i)); 
      i += 1; 
    } 

    sorted_listI = mergesort (listI); 
    sorted_listII = mergesort (listII); 

    return mergesort_merge (sorted_listI, sorted_listII); 

  } 

} 


object QuickSort { 

  def quicksort (sortee: Vector[Int]): Vector[Int] = { 

    if (sortee.length <= 1) { 
      return sortee; 
    } 
    
    // nextInt is exclusive, so we're not off-by-one 
    val pivot_index = Random.nextInt(sortee.length); 
    val pivot = sortee(pivot_index); 
    var leftside = Vector[Int](); 
    var rightside = Vector[Int](); 

    for (i: Int <- sortee.slice(0,pivot_index)) { 
      if (i < pivot) { 
        leftside ++= Vector(i); 
      } else { 
        rightside ++= Vector(i); 
      } 
    } 
    for (i: Int <- sortee.slice(pivot_index+1,sortee.length)) { 
      if (i < pivot) { 
        leftside ++= Vector(i); 
      } else { 
        rightside ++= Vector(i); 
      } 
    } 

    return quicksort(leftside) ++ Vector(pivot) ++ quicksort(rightside); 

  } 

} 
