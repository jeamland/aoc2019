(ns day6-2.core
  (:gen-class))

(defn source-lines [] (line-seq (java.io.BufferedReader. *in*)))

(defn orbit-map [source]
  (into {} (map (fn [s] (vec (reverse (clojure.string/split s #"\)")))) source)))

(defn orbit-list [body orbits & s]
  (if (= body "COM")
    s
    (recur (orbits body) orbits (concat s [body]))))

(defn find-junction [orbit1 orbit2 & prev]
  (if (not= (first orbit1) (first orbit2))
    prev
    (recur (rest orbit1) (rest orbit2) (first orbit1))))

(defn orbit-distance [to list]
  (count (take-while (fn [x] (not= x to)) (rest list))))

(defn -main
  [& _args]
  (let [orbits (orbit-map (source-lines))
        you-orbits (orbit-list "YOU" orbits)
        san-orbits (orbit-list "SAN" orbits)
        junction (find-junction (reverse you-orbits) (reverse san-orbits))]
    (println (+ (orbit-distance junction you-orbits) (orbit-distance junction san-orbits)))))
