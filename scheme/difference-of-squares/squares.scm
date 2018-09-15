(define-module (squares)
  #:export (sum-of-squares
            square-of-sum
            difference)
  #:autoload (srfi srfi-1) (reduce iota))

(define (sum n)
  (if (= n 0)
      n
      (+ n (sum (- n 1)))))

(define (square-of-sum n) (expt (sum n) 2))

(define (sum-of-squares n)
  (if (= n 0)
      n
      (+ (expt n 2) (sum-of-squares (- n 1)))))

(define (difference n)
  (- (square-of-sum n) (sum-of-squares n)))