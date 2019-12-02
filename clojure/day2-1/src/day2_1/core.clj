(ns day2-1.core
  (:gen-class))

(defn value_at_addr [memory addr]
  (nth memory (nth memory addr)))

(defn op_add [memory pc]
  (assoc memory (nth memory (+ pc 3)) (+ (value_at_addr memory (+ pc 1)) (value_at_addr memory (+ pc 2)))))

(defn op_mul [memory pc]
  (assoc memory (nth memory (+ pc 3)) (* (value_at_addr memory (+ pc 1)) (value_at_addr memory (+ pc 2)))))

(defn execute [memory pc]
  (case (nth memory pc)
    1 (execute (op_add memory pc) (+ pc 4))
    2 (execute (op_mul memory pc) (+ pc 4))
    99 memory
    "wtf"))

(defn -main
  [& _args]
  (def memory (mapv #(Integer/parseInt %) (clojure.string/split (clojure.string/trim-newline (slurp *in*)) #",")))
  (println (execute memory 0)))
