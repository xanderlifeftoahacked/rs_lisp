(def i 10)

(def cmd-1 ('(set! i (- i 1))))

(eval cmd-1)

(def func ('(cond (< i 1) (print "end") (do (set! r (* r i)) (set! i (- i 1)) (eval func) ))))
