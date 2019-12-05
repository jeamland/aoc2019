(ns day5-2.core
  (:gen-class))

(defn split-modes [n]
  (->> n
       (iterate #(quot % 10))
       (take-while pos?)
       (mapv #(mod % 10))
       seq))

(defn decompose-op [memory pc]
  (let [op (nth memory pc)]
    [(mod op 100) (split-modes (int (Math/floor (/ op 100))))]))

(defn value_at_addr [memory addr mode]
  (case mode
    0 (nth memory (nth memory addr))
    1 (nth memory addr)
    "wtf addr"))

(defn op_3 [memory pc modes func]
  (assoc memory
         (nth memory (+ pc 3))
         (func (value_at_addr memory (+ pc 1) (nth modes 0 0))
               (value_at_addr memory (+ pc 2) (nth modes 1 0)))))

(defn read-stdin [memory pc]
  (print "> ")
  (flush)
  (let [input (first (line-seq (java.io.BufferedReader. *in*)))]
    (assoc memory (nth memory (+ pc 1)) (Integer/parseInt input))))

(defn write-stdout [memory pc mode]
  (println "#" (value_at_addr memory (+ pc 1) mode))
  memory)

(defn compare-equal [a b]
  (if (= a b) 1 0))

(defn compare-less [a b]
  (if (< a b) 1 0))

(defn execute [memory pc]
  (let [op (decompose-op memory pc) opcode (first op) modes (second op)]
    (case opcode
      1 (recur (op_3 memory pc modes +) (+ pc 4))
      2 (recur (op_3 memory pc modes *) (+ pc 4))
      3 (recur (read-stdin memory pc) (+ pc 2))
      4 (recur (write-stdout memory pc (nth modes 0 0)) (+ pc 2))
      5 (recur memory
               (if (not= 0 (value_at_addr memory (+ pc 1) (nth modes 0 0)))
                 (value_at_addr memory (+ pc 2) (nth modes 1 0))
                 (+ pc 3)))
      6 (recur memory
               (if (zero? (value_at_addr memory (+ pc 1) (nth modes 0 0)))
                 (value_at_addr memory (+ pc 2) (nth modes 1 0))
                 (+ pc 3)))
      7 (recur (op_3 memory pc modes compare-less) (+ pc 4))
      8 (recur (op_3 memory pc modes compare-equal) (+ pc 4))
      99 memory
      "wtf opcode")))

(defn -main
  [& args]
  (let [memory (mapv #(Integer/parseInt %) (clojure.string/split (clojure.string/trim-newline (slurp (first args))) #","))]
    (println (execute memory 0))))