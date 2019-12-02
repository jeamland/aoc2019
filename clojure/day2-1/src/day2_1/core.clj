(ns day2-1.core
  (:gen-class))

(defn value_at_addr [memory addr]
  (nth memory (nth memory addr)))

(defn op_quad [memory pc func]
  (assoc memory (nth memory (+ pc 3)) (func (value_at_addr memory (+ pc 1)) (value_at_addr memory (+ pc 2)))))

(defn execute [memory pc]
  (case (nth memory pc)
    1 (execute (op_quad memory pc +) (+ pc 4))
    2 (execute (op_quad memory pc *) (+ pc 4))
    99 memory
    "wtf"))

(defn -main
  [& _args]
  (let [memory (mapv #(Integer/parseInt %) (clojure.string/split (clojure.string/trim-newline (slurp *in*)) #","))]
    (println (execute memory 0))))
