(define-module (hamming)
  #:export (hamming-distance))



(define (count-diff one two count)
  (if (equal? one '())
    count
    (if (not(char=? (car one) (car two))) 
      (count-diff (cdr one) (cdr two) (+ count 1)) 
      (count-diff (cdr one) (cdr two) count))
  )
)

(define (hamming-distance one two)
  (count-diff (string->list one) (string->list two) 0))