(ns day4-2.core
  (:gen-class))

(defn digits [n]
  (->> n
       (iterate #(quot % 10))
       (take-while pos?)
       (mapv #(mod % 10))
       rseq))

(defn non-descending [digits]
  (empty? (filter (fn [x] (> (first x) (last x))) (partition 2 1 digits))))

(defn at-least-one-pair [digits]
  (seq (filter (fn [x] (= 2 (last x))) (seq (frequencies digits)))))

(defn -main
  [& _args]
  (let [start 231832 end 767346]
    (println
     (count
      (filter at-least-one-pair
              (filter non-descending
                      (map digits (range start (+ end 1)))))))))
