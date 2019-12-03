(ns day2-2.core
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

(defn testrun [memory noun verb]
  (let [test_memory (execute (assoc (assoc memory 1 noun) 2 verb) 0)]
    [(nth test_memory 0) noun verb]))

(defn testvalues []
  (for [noun (range 100) verb (range 100)] [noun verb]))

(defn testruns [memory]
  (map (fn [p] (testrun memory (nth p 0) (nth p 1))) (testvalues)))

(defn -main
  [& _args]
  (let [memory (mapv #(Integer/parseInt %) (clojure.string/split (clojure.string/trim-newline (slurp *in*)) #","))]
    (println (filter (fn [v] (= (nth v 0) 19690720)) (testruns memory)))))
