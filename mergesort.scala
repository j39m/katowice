object MergeSort { 


  def main (args: Array[String]) { 
    val klaus : Vector[Int] = Vector(17,18,0,3,2,1,5,17,13,14,11); 
    val tama : Vector[Int] = sort(klaus); 
    println(tama); 
    return; 
  } 


  def merge (listI: Vector[Int], listII: Vector[Int]): Vector[Int] = { 

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


  def sort (sortee: Vector[Int]): Vector[Int] = { 

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

    sorted_listI = sort (listI); 
    sorted_listII = sort (listII); 

    return merge (sorted_listI, sorted_listII); 

  } 


} 

